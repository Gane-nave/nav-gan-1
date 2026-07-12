/**
 * G.A.N.E — Error-State Kalman Filter (ESKF)
 * ============================================
 * Fuses GNSS, IMU, and Vision data for optimal position estimation.
 * Implements the full 15-state error-state formulation:
 *   δx = [δp(3), δv(3), δθ(3), δba(3), δbg(3)]
 *
 * Reference: Solà, "Quaternion kinematics for the error-state Kalman filter"
 */

// ─── Types ───
export interface Vec3 { x: number; y: number; z: number; }
export interface Quaternion { w: number; x: number; y: number; z: number; }

export interface GNSSMeasurement {
  lat: number;
  lon: number;
  alt: number;
  accuracy: number;      // meters (CEP)
  timestamp: number;
  satellites: number;
  hdop: number;
}

export interface IMUMeasurement {
  accel: Vec3;           // m/s² in body frame
  gyro: Vec3;            // rad/s in body frame
  timestamp: number;
  temperature: number;   // °C (for bias compensation)
}

export interface VisionMeasurement {
  deltaPosition: Vec3;   // visual odometry displacement
  confidence: number;    // 0-1
  featureCount: number;
  timestamp: number;
}

export interface ESKFState {
  position: Vec3;        // ECEF or local ENU (meters)
  velocity: Vec3;        // m/s
  orientation: Quaternion;
  accelBias: Vec3;       // m/s²
  gyroBias: Vec3;        // rad/s
  timestamp: number;
  confidence: number;    // 0-1 overall confidence
  mode: 'OPTIMAL_FUSION' | 'DEGRADED_MODE' | 'EMERGENCY_BOUNDED';
}

// ─── Matrix utilities (inline for zero-dependency) ───
type Mat = number[][];

function zeros(r: number, c: number): Mat {
  return Array.from({ length: r }, () => new Array(c).fill(0));
}

function eye(n: number): Mat {
  const m = zeros(n, n);
  for (let i = 0; i < n; i++) m[i][i] = 1;
  return m;
}

function matMul(A: Mat, B: Mat): Mat {
  const r = A.length, c = B[0].length, k = B.length;
  const C = zeros(r, c);
  for (let i = 0; i < r; i++)
    for (let j = 0; j < c; j++)
      for (let l = 0; l < k; l++)
        C[i][j] += A[i][l] * B[l][j];
  return C;
}

function matAdd(A: Mat, B: Mat): Mat {
  return A.map((row, i) => row.map((v, j) => v + B[i][j]));
}

function matSub(A: Mat, B: Mat): Mat {
  return A.map((row, i) => row.map((v, j) => v - B[i][j]));
}

function transpose(A: Mat): Mat {
  const r = A.length, c = A[0].length;
  const T = zeros(c, r);
  for (let i = 0; i < r; i++)
    for (let j = 0; j < c; j++)
      T[j][i] = A[i][j];
  return T;
}

function matScale(A: Mat, s: number): Mat {
  return A.map(row => row.map(v => v * s));
}

/** 3x3 matrix inverse (Cramer's rule) */
function inv3(m: Mat): Mat {
  const [a, b, c] = m[0], [d, e, f] = m[1], [g, h, k] = m[2];
  const det = a * (e * k - f * h) - b * (d * k - f * g) + c * (d * h - e * g);
  if (Math.abs(det) < 1e-12) return eye(3);
  const invDet = 1 / det;
  return [
    [(e * k - f * h) * invDet, (c * h - b * k) * invDet, (b * f - c * e) * invDet],
    [(f * g - d * k) * invDet, (a * k - c * g) * invDet, (c * d - a * f) * invDet],
    [(d * h - e * g) * invDet, (b * g - a * h) * invDet, (a * e - b * d) * invDet],
  ];
}

/** Generalized matrix inverse using block decomposition for small matrices */
function matInv(A: Mat): Mat {
  const n = A.length;
  if (n === 3) return inv3(A);
  // Gauss-Jordan for general case
  const aug = A.map((row, i) => [...row, ...eye(n)[i]]);
  for (let col = 0; col < n; col++) {
    let maxRow = col;
    for (let row = col + 1; row < n; row++)
      if (Math.abs(aug[row][col]) > Math.abs(aug[maxRow][col])) maxRow = row;
    [aug[col], aug[maxRow]] = [aug[maxRow], aug[col]];
    const pivot = aug[col][col];
    if (Math.abs(pivot) < 1e-12) continue;
    for (let j = 0; j < 2 * n; j++) aug[col][j] /= pivot;
    for (let row = 0; row < n; row++) {
      if (row === col) continue;
      const factor = aug[row][col];
      for (let j = 0; j < 2 * n; j++) aug[row][j] -= factor * aug[col][j];
    }
  }
  return aug.map(row => row.slice(n));
}

// ─── Quaternion operations ───
function quatMul(a: Quaternion, b: Quaternion): Quaternion {
  return {
    w: a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
    x: a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
    y: a.w * b.y - a.x * b.z + a.y * b.w + a.z * b.x,
    z: a.w * b.z + a.x * b.y - a.y * b.x + a.z * b.w,
  };
}

function quatNormalize(q: Quaternion): Quaternion {
  const n = Math.sqrt(q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z);
  if (n < 1e-12) return { w: 1, x: 0, y: 0, z: 0 };
  return { w: q.w / n, x: q.x / n, y: q.y / n, z: q.z / n };
}

function quatToRotMat(q: Quaternion): Mat {
  const { w, x, y, z } = q;
  return [
    [1 - 2 * (y * y + z * z), 2 * (x * y - w * z), 2 * (x * z + w * y)],
    [2 * (x * y + w * z), 1 - 2 * (x * x + z * z), 2 * (y * z - w * x)],
    [2 * (x * z - w * y), 2 * (y * z + w * x), 1 - 2 * (x * x + y * y)],
  ];
}

function smallAngleQuat(dtheta: Vec3): Quaternion {
  const angle = Math.sqrt(dtheta.x ** 2 + dtheta.y ** 2 + dtheta.z ** 2);
  if (angle < 1e-8) return { w: 1, x: dtheta.x / 2, y: dtheta.y / 2, z: dtheta.z / 2 };
  const ha = angle / 2;
  const s = Math.sin(ha) / angle;
  return quatNormalize({ w: Math.cos(ha), x: dtheta.x * s, y: dtheta.y * s, z: dtheta.z * s });
}

function skewSymmetric(v: Vec3): Mat {
  return [
    [0, -v.z, v.y],
    [v.z, 0, -v.x],
    [-v.y, v.x, 0],
  ];
}

// ─── Coordinate transforms ───
const WGS84_A = 6378137.0;
const WGS84_E2 = 0.00669437999014;

export function geodeticToENU(lat: number, lon: number, alt: number, refLat: number, refLon: number, refAlt: number): Vec3 {
  const toRad = Math.PI / 180;
  const dLat = (lat - refLat) * toRad;
  const dLon = (lon - refLon) * toRad;
  const sinRefLat = Math.sin(refLat * toRad);
  const cosRefLat = Math.cos(refLat * toRad);
  const N = WGS84_A / Math.sqrt(1 - WGS84_E2 * sinRefLat * sinRefLat);
  return {
    x: dLon * (N + refAlt) * cosRefLat,  // East
    y: dLat * (N * (1 - WGS84_E2) + refAlt), // North
    z: alt - refAlt,                       // Up
  };
}

export function enuToGeodetic(enu: Vec3, refLat: number, refLon: number, refAlt: number): { lat: number; lon: number; alt: number } {
  const toRad = Math.PI / 180;
  const sinRefLat = Math.sin(refLat * toRad);
  const cosRefLat = Math.cos(refLat * toRad);
  const N = WGS84_A / Math.sqrt(1 - WGS84_E2 * sinRefLat * sinRefLat);
  return {
    lat: refLat + (enu.y / (N * (1 - WGS84_E2) + refAlt)) / toRad,
    lon: refLon + (enu.x / ((N + refAlt) * cosRefLat)) / toRad,
    alt: refAlt + enu.z,
  };
}

// ─── ESKF Engine ───
const STATE_DIM = 15; // δp(3) + δv(3) + δθ(3) + δba(3) + δbg(3)
const GRAVITY: Vec3 = { x: 0, y: 0, z: -9.80665 };

// Process noise parameters (tunable)
const ACCEL_NOISE = 0.1;        // m/s² √Hz
const GYRO_NOISE = 0.01;        // rad/s √Hz
const ACCEL_BIAS_WALK = 0.001;  // m/s² √Hz
const GYRO_BIAS_WALK = 0.0001;  // rad/s √Hz

export class ESKFEngine {
  private state: ESKFState;
  private P: Mat;                // 15x15 error covariance
  private refLat: number;
  private refLon: number;
  private refAlt: number;
  private lastIMUTimestamp: number = 0;
  private gnssLossStart: number = 0;
  private consecutiveGNSSRejects: number = 0;

  constructor(initialGNSS?: GNSSMeasurement) {
    this.refLat = initialGNSS?.lat ?? 32.0853;
    this.refLon = initialGNSS?.lon ?? 34.7818;
    this.refAlt = initialGNSS?.alt ?? 50;

    this.state = {
      position: { x: 0, y: 0, z: 0 },
      velocity: { x: 0, y: 0, z: 0 },
      orientation: { w: 1, x: 0, y: 0, z: 0 },
      accelBias: { x: 0, y: 0, z: 0 },
      gyroBias: { x: 0, y: 0, z: 0 },
      timestamp: initialGNSS?.timestamp ?? Date.now(),
      confidence: 1.0,
      mode: 'OPTIMAL_FUSION',
    };

    // Initial covariance
    this.P = eye(STATE_DIM);
    // Position uncertainty: 10m
    this.P[0][0] = this.P[1][1] = this.P[2][2] = 100;
    // Velocity uncertainty: 1 m/s
    this.P[3][3] = this.P[4][4] = this.P[5][5] = 1;
    // Orientation uncertainty: 5 degrees
    this.P[6][6] = this.P[7][7] = this.P[8][8] = (5 * Math.PI / 180) ** 2;
    // Bias uncertainties
    this.P[9][9] = this.P[10][10] = this.P[11][11] = 0.01;
    this.P[12][12] = this.P[13][13] = this.P[14][14] = 0.001;
  }

  /** IMU Prediction Step (high frequency: 100Hz+) */
  predictIMU(imu: IMUMeasurement): void {
    if (this.lastIMUTimestamp === 0) {
      this.lastIMUTimestamp = imu.timestamp;
      return;
    }

    const dt = (imu.timestamp - this.lastIMUTimestamp) / 1000;
    if (dt <= 0 || dt > 1) {
      this.lastIMUTimestamp = imu.timestamp;
      return;
    }
    this.lastIMUTimestamp = imu.timestamp;

    // Corrected IMU readings (subtract bias)
    const accelCorr: Vec3 = {
      x: imu.accel.x - this.state.accelBias.x,
      y: imu.accel.y - this.state.accelBias.y,
      z: imu.accel.z - this.state.accelBias.z,
    };
    const gyroCorr: Vec3 = {
      x: imu.gyro.x - this.state.gyroBias.x,
      y: imu.gyro.y - this.state.gyroBias.y,
      z: imu.gyro.z - this.state.gyroBias.z,
    };

    // Rotate acceleration to navigation frame
    const R = quatToRotMat(this.state.orientation);
    const accelNav: Vec3 = {
      x: R[0][0] * accelCorr.x + R[0][1] * accelCorr.y + R[0][2] * accelCorr.z + GRAVITY.x,
      y: R[1][0] * accelCorr.x + R[1][1] * accelCorr.y + R[1][2] * accelCorr.z + GRAVITY.y,
      z: R[2][0] * accelCorr.x + R[2][1] * accelCorr.y + R[2][2] * accelCorr.z + GRAVITY.z,
    };

    // Nominal state propagation
    this.state.position.x += this.state.velocity.x * dt + 0.5 * accelNav.x * dt * dt;
    this.state.position.y += this.state.velocity.y * dt + 0.5 * accelNav.y * dt * dt;
    this.state.position.z += this.state.velocity.z * dt + 0.5 * accelNav.z * dt * dt;

    this.state.velocity.x += accelNav.x * dt;
    this.state.velocity.y += accelNav.y * dt;
    this.state.velocity.z += accelNav.z * dt;

    // Orientation update
    const dtheta: Vec3 = { x: gyroCorr.x * dt, y: gyroCorr.y * dt, z: gyroCorr.z * dt };
    this.state.orientation = quatNormalize(quatMul(this.state.orientation, smallAngleQuat(dtheta)));

    // Error-state transition matrix F (15x15)
    const F = eye(STATE_DIM);
    // δp += δv * dt
    F[0][3] = dt; F[1][4] = dt; F[2][5] = dt;
    // δv += -R*[a×]*δθ*dt - R*δba*dt
    const aSkew = skewSymmetric(accelCorr);
    const RaSkew = matMul(R, aSkew);
    for (let i = 0; i < 3; i++) {
      for (let j = 0; j < 3; j++) {
        F[3 + i][6 + j] = -RaSkew[i][j] * dt;
        F[3 + i][9 + j] = -R[i][j] * dt;
      }
    }
    // δθ += -δbg * dt
    F[6][12] = -dt; F[7][13] = -dt; F[8][14] = -dt;

    // Process noise Q
    const Q = zeros(STATE_DIM, STATE_DIM);
    const an2 = ACCEL_NOISE * ACCEL_NOISE * dt;
    const gn2 = GYRO_NOISE * GYRO_NOISE * dt;
    const abw2 = ACCEL_BIAS_WALK * ACCEL_BIAS_WALK * dt;
    const gbw2 = GYRO_BIAS_WALK * GYRO_BIAS_WALK * dt;
    for (let i = 3; i < 6; i++) Q[i][i] = an2;
    for (let i = 6; i < 9; i++) Q[i][i] = gn2;
    for (let i = 9; i < 12; i++) Q[i][i] = abw2;
    for (let i = 12; i < 15; i++) Q[i][i] = gbw2;

    // Covariance propagation: P = F*P*F' + Q
    this.P = matAdd(matMul(matMul(F, this.P), transpose(F)), Q);

    this.state.timestamp = imu.timestamp;
  }

  /** GNSS Update Step (1-10Hz) */
  updateGNSS(gnss: GNSSMeasurement): { accepted: boolean; residual: number; chiSquare: number } {
    const enu = geodeticToENU(gnss.lat, gnss.lon, gnss.alt, this.refLat, this.refLon, this.refAlt);

    // Innovation (measurement residual)
    const innovation = [
      enu.x - this.state.position.x,
      enu.y - this.state.position.y,
      enu.z - this.state.position.z,
    ];

    // Observation matrix H (3x15): measures position
    const H = zeros(3, STATE_DIM);
    H[0][0] = 1; H[1][1] = 1; H[2][2] = 1;

    // Measurement noise R (based on GNSS accuracy + HDOP)
    const sigmaPos = gnss.accuracy * Math.max(gnss.hdop, 1.0);
    const R_meas = zeros(3, 3);
    R_meas[0][0] = R_meas[1][1] = sigmaPos * sigmaPos;
    R_meas[2][2] = (sigmaPos * 2) * (sigmaPos * 2); // vertical is worse

    // Innovation covariance S = H*P*H' + R
    const S = matAdd(matMul(matMul(H, this.P), transpose(H)), R_meas);

    // ─── FDE: Chi-Square Residual Test ───
    const S_inv = inv3(S);
    const innovVec = [[innovation[0]], [innovation[1]], [innovation[2]]];
    const chiSquareVal = matMul(matMul(transpose(innovVec), S_inv), innovVec)[0][0];

    // Chi-Square threshold for 3 DOF at 99.5% confidence
    const CHI2_THRESHOLD = 12.838;

    if (chiSquareVal > CHI2_THRESHOLD) {
      // Spoofing/jamming detected — reject this measurement
      this.consecutiveGNSSRejects++;
      if (this.consecutiveGNSSRejects >= 3) {
        this.state.mode = 'DEGRADED_MODE';
        this.state.confidence = Math.max(0.1, this.state.confidence - 0.15);
      }
      return { accepted: false, residual: Math.sqrt(innovation[0] ** 2 + innovation[1] ** 2 + innovation[2] ** 2), chiSquare: chiSquareVal };
    }

    // Reset reject counter
    this.consecutiveGNSSRejects = 0;
    this.gnssLossStart = 0;

    // Kalman Gain K = P*H'*S^-1
    const K = matMul(matMul(this.P, transpose(H)), matInv(S));

    // State correction
    const dx = matMul(K, innovVec);

    // Apply error-state correction
    this.state.position.x += dx[0][0];
    this.state.position.y += dx[1][0];
    this.state.position.z += dx[2][0];
    this.state.velocity.x += dx[3][0];
    this.state.velocity.y += dx[4][0];
    this.state.velocity.z += dx[5][0];

    // Orientation correction
    const dtheta: Vec3 = { x: dx[6][0], y: dx[7][0], z: dx[8][0] };
    this.state.orientation = quatNormalize(quatMul(this.state.orientation, smallAngleQuat(dtheta)));

    // Bias corrections
    this.state.accelBias.x += dx[9][0];
    this.state.accelBias.y += dx[10][0];
    this.state.accelBias.z += dx[11][0];
    this.state.gyroBias.x += dx[12][0];
    this.state.gyroBias.y += dx[13][0];
    this.state.gyroBias.z += dx[14][0];

    // Covariance update: P = (I - K*H)*P
    this.P = matMul(matSub(eye(STATE_DIM), matMul(K, H)), this.P);

    // Update confidence
    this.state.confidence = Math.min(1.0, this.state.confidence + 0.05);
    if (gnss.satellites >= 6 && gnss.accuracy < 5) {
      this.state.mode = 'OPTIMAL_FUSION';
    }

    this.state.timestamp = gnss.timestamp;
    return { accepted: true, residual: Math.sqrt(innovation[0] ** 2 + innovation[1] ** 2 + innovation[2] ** 2), chiSquare: chiSquareVal };
  }

  /** Vision Update Step (visual odometry) */
  updateVision(vision: VisionMeasurement): void {
    if (vision.confidence < 0.3 || vision.featureCount < 10) return;

    // Vision provides relative displacement in body frame
    const R = quatToRotMat(this.state.orientation);
    const dp = vision.deltaPosition;
    const dpNav: Vec3 = {
      x: R[0][0] * dp.x + R[0][1] * dp.y + R[0][2] * dp.z,
      y: R[1][0] * dp.x + R[1][1] * dp.y + R[1][2] * dp.z,
      z: R[2][0] * dp.x + R[2][1] * dp.y + R[2][2] * dp.z,
    };

    const innovation = [dpNav.x, dpNav.y, dpNav.z];
    const H = zeros(3, STATE_DIM);
    H[0][0] = 1; H[1][1] = 1; H[2][2] = 1;

    const sigma = 0.5 / vision.confidence;
    const R_meas = matScale(eye(3), sigma * sigma);
    const S = matAdd(matMul(matMul(H, this.P), transpose(H)), R_meas);
    const K = matMul(matMul(this.P, transpose(H)), matInv(S));
    const innovVec = [[innovation[0]], [innovation[1]], [innovation[2]]];
    const dx = matMul(K, innovVec);

    this.state.position.x += dx[0][0];
    this.state.position.y += dx[1][0];
    this.state.position.z += dx[2][0];

    this.P = matMul(matSub(eye(STATE_DIM), matMul(K, H)), this.P);
  }

  /** Get current state in geodetic coordinates */
  getGeodeticState(): { lat: number; lon: number; alt: number; heading: number; speed: number; confidence: number; mode: string } {
    const geo = enuToGeodetic(this.state.position, this.refLat, this.refLon, this.refAlt);
    const speed = Math.sqrt(this.state.velocity.x ** 2 + this.state.velocity.y ** 2);
    const heading = (Math.atan2(this.state.velocity.x, this.state.velocity.y) * 180 / Math.PI + 360) % 360;
    return {
      ...geo,
      heading,
      speed,
      confidence: this.state.confidence,
      mode: this.state.mode,
    };
  }

  /** Get raw ENU state */
  getState(): ESKFState {
    return { ...this.state };
  }

  /** Get position uncertainty (1-sigma, meters) */
  getPositionUncertainty(): { horizontal: number; vertical: number } {
    return {
      horizontal: Math.sqrt(this.P[0][0] + this.P[1][1]),
      vertical: Math.sqrt(this.P[2][2]),
    };
  }

  /** Signal GNSS loss for dead reckoning mode */
  signalGNSSLoss(): void {
    if (this.gnssLossStart === 0) this.gnssLossStart = Date.now();
    const lossSeconds = (Date.now() - this.gnssLossStart) / 1000;
    if (lossSeconds > 5) {
      this.state.mode = 'DEGRADED_MODE';
      this.state.confidence = Math.max(0.2, 1.0 - lossSeconds * 0.02);
    }
    if (lossSeconds > 60) {
      this.state.mode = 'EMERGENCY_BOUNDED';
      this.state.confidence = Math.max(0.05, 0.2 - (lossSeconds - 60) * 0.005);
    }
  }
}
