/**
 * G.A.N.E — MATHEMATICAL MODELS
 * 
 * Spec Reference: Doc 7 §3-5, Doc 8 §2-4
 * 
 * Formal mathematical foundations:
 * 1. Covariance propagation for sensor fusion
 * 2. Bayesian trust fusion for incident validation
 * 3. Multi-objective route optimization
 * 4. Uncertainty bounds and stability proofs
 * 5. State-space models for ESKF
 */

// ═══════════════════════════════════════════════════════════
// 1. MATRIX OPERATIONS — Foundation for all math
// ═══════════════════════════════════════════════════════════

/**
 * Dense matrix stored as flat Float64Array in row-major order.
 * All operations are allocation-free where possible.
 */
export class Matrix {
  readonly data: Float64Array;
  readonly rows: number;
  readonly cols: number;

  constructor(rows: number, cols: number, data?: Float64Array | number[]) {
    this.rows = rows;
    this.cols = cols;
    this.data = data
      ? (data instanceof Float64Array ? data : new Float64Array(data))
      : new Float64Array(rows * cols);
  }

  static identity(n: number): Matrix {
    const m = new Matrix(n, n);
    for (let i = 0; i < n; i++) m.data[i * n + i] = 1;
    return m;
  }

  static zeros(rows: number, cols: number): Matrix {
    return new Matrix(rows, cols);
  }

  static diag(values: number[]): Matrix {
    const n = values.length;
    const m = new Matrix(n, n);
    for (let i = 0; i < n; i++) m.data[i * n + i] = values[i];
    return m;
  }

  get(r: number, c: number): number {
    return this.data[r * this.cols + c];
  }

  set(r: number, c: number, v: number): void {
    this.data[r * this.cols + c] = v;
  }

  /** C = A * B */
  multiply(b: Matrix): Matrix {
    if (this.cols !== b.rows) throw new Error(`Matrix multiply: ${this.cols} !== ${b.rows}`);
    const result = new Matrix(this.rows, b.cols);
    for (let i = 0; i < this.rows; i++) {
      for (let k = 0; k < this.cols; k++) {
        const aik = this.data[i * this.cols + k];
        if (aik === 0) continue;
        for (let j = 0; j < b.cols; j++) {
          result.data[i * b.cols + j] += aik * b.data[k * b.cols + j];
        }
      }
    }
    return result;
  }

  /** C = A + B */
  add(b: Matrix): Matrix {
    if (this.rows !== b.rows || this.cols !== b.cols) throw new Error('Matrix add: dimension mismatch');
    const result = new Matrix(this.rows, this.cols);
    for (let i = 0; i < this.data.length; i++) {
      result.data[i] = this.data[i] + b.data[i];
    }
    return result;
  }

  /** C = A - B */
  subtract(b: Matrix): Matrix {
    if (this.rows !== b.rows || this.cols !== b.cols) throw new Error('Matrix subtract: dimension mismatch');
    const result = new Matrix(this.rows, this.cols);
    for (let i = 0; i < this.data.length; i++) {
      result.data[i] = this.data[i] - b.data[i];
    }
    return result;
  }

  /** Transpose */
  transpose(): Matrix {
    const result = new Matrix(this.cols, this.rows);
    for (let i = 0; i < this.rows; i++) {
      for (let j = 0; j < this.cols; j++) {
        result.data[j * this.rows + i] = this.data[i * this.cols + j];
      }
    }
    return result;
  }

  /** Scalar multiply */
  scale(s: number): Matrix {
    const result = new Matrix(this.rows, this.cols);
    for (let i = 0; i < this.data.length; i++) {
      result.data[i] = this.data[i] * s;
    }
    return result;
  }

  /** Trace (sum of diagonal) */
  trace(): number {
    let sum = 0;
    const n = Math.min(this.rows, this.cols);
    for (let i = 0; i < n; i++) sum += this.data[i * this.cols + i];
    return sum;
  }

  /** Determinant (2x2 and 3x3 only) */
  determinant(): number {
    if (this.rows !== this.cols) throw new Error('Determinant requires square matrix');
    if (this.rows === 1) return this.data[0];
    if (this.rows === 2) {
      return this.data[0] * this.data[3] - this.data[1] * this.data[2];
    }
    if (this.rows === 3) {
      const [a, b, c, d, e, f, g, h, i] = Array.from(this.data);
      return a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
    }
    // LU decomposition for larger matrices
    return this.luDeterminant();
  }

  private luDeterminant(): number {
    const n = this.rows;
    const lu = new Float64Array(this.data);
    let det = 1;
    for (let i = 0; i < n; i++) {
      let maxVal = Math.abs(lu[i * n + i]);
      let maxRow = i;
      for (let k = i + 1; k < n; k++) {
        if (Math.abs(lu[k * n + i]) > maxVal) {
          maxVal = Math.abs(lu[k * n + i]);
          maxRow = k;
        }
      }
      if (maxRow !== i) {
        for (let j = 0; j < n; j++) {
          const tmp = lu[i * n + j];
          lu[i * n + j] = lu[maxRow * n + j];
          lu[maxRow * n + j] = tmp;
        }
        det *= -1;
      }
      if (Math.abs(lu[i * n + i]) < 1e-12) return 0;
      det *= lu[i * n + i];
      for (let k = i + 1; k < n; k++) {
        lu[k * n + i] /= lu[i * n + i];
        for (let j = i + 1; j < n; j++) {
          lu[k * n + j] -= lu[k * n + i] * lu[i * n + j];
        }
      }
    }
    return det;
  }

  /** Inverse (for small matrices via Gauss-Jordan) */
  inverse(): Matrix {
    if (this.rows !== this.cols) throw new Error('Inverse requires square matrix');
    const n = this.rows;
    const aug = new Float64Array(n * 2 * n);

    // Build augmented matrix [A | I]
    for (let i = 0; i < n; i++) {
      for (let j = 0; j < n; j++) {
        aug[i * 2 * n + j] = this.data[i * n + j];
      }
      aug[i * 2 * n + n + i] = 1;
    }

    // Gauss-Jordan elimination
    for (let i = 0; i < n; i++) {
      let maxVal = Math.abs(aug[i * 2 * n + i]);
      let maxRow = i;
      for (let k = i + 1; k < n; k++) {
        if (Math.abs(aug[k * 2 * n + i]) > maxVal) {
          maxVal = Math.abs(aug[k * 2 * n + i]);
          maxRow = k;
        }
      }
      if (maxRow !== i) {
        for (let j = 0; j < 2 * n; j++) {
          const tmp = aug[i * 2 * n + j];
          aug[i * 2 * n + j] = aug[maxRow * 2 * n + j];
          aug[maxRow * 2 * n + j] = tmp;
        }
      }

      const pivot = aug[i * 2 * n + i];
      if (Math.abs(pivot) < 1e-12) throw new Error('Matrix is singular');

      for (let j = 0; j < 2 * n; j++) aug[i * 2 * n + j] /= pivot;

      for (let k = 0; k < n; k++) {
        if (k === i) continue;
        const factor = aug[k * 2 * n + i];
        for (let j = 0; j < 2 * n; j++) {
          aug[k * 2 * n + j] -= factor * aug[i * 2 * n + j];
        }
      }
    }

    // Extract inverse
    const result = new Matrix(n, n);
    for (let i = 0; i < n; i++) {
      for (let j = 0; j < n; j++) {
        result.data[i * n + j] = aug[i * 2 * n + n + j];
      }
    }
    return result;
  }

  /** Cholesky decomposition (for positive definite matrices) */
  cholesky(): Matrix {
    if (this.rows !== this.cols) throw new Error('Cholesky requires square matrix');
    const n = this.rows;
    const L = new Matrix(n, n);

    for (let i = 0; i < n; i++) {
      for (let j = 0; j <= i; j++) {
        let sum = 0;
        for (let k = 0; k < j; k++) {
          sum += L.data[i * n + k] * L.data[j * n + k];
        }
        if (i === j) {
          const val = this.data[i * n + i] - sum;
          if (val <= 0) throw new Error('Matrix is not positive definite');
          L.data[i * n + j] = Math.sqrt(val);
        } else {
          L.data[i * n + j] = (this.data[i * n + j] - sum) / L.data[j * n + j];
        }
      }
    }
    return L;
  }

  /** Check if positive definite */
  isPositiveDefinite(): boolean {
    try {
      this.cholesky();
      return true;
    } catch {
      return false;
    }
  }

  /** Frobenius norm */
  frobeniusNorm(): number {
    let sum = 0;
    for (let i = 0; i < this.data.length; i++) {
      sum += this.data[i] * this.data[i];
    }
    return Math.sqrt(sum);
  }

  clone(): Matrix {
    return new Matrix(this.rows, this.cols, new Float64Array(this.data));
  }
}

// ═══════════════════════════════════════════════════════════
// 2. COVARIANCE PROPAGATION — ESKF State-Space Model
// ═══════════════════════════════════════════════════════════

/**
 * ESKF State Vector (15-state):
 * [δp_x, δp_y, δp_z, δv_x, δv_y, δv_z, δθ_x, δθ_y, δθ_z, b_ax, b_ay, b_az, b_gx, b_gy, b_gz]
 * 
 * Position error (3), Velocity error (3), Attitude error (3),
 * Accelerometer bias (3), Gyroscope bias (3)
 */
export const ESKF_STATE_DIM = 15;

export interface ESKFConfig {
  /** Process noise spectral densities */
  accel_noise_density: number;    // m/s²/√Hz
  gyro_noise_density: number;     // rad/s/√Hz
  accel_bias_instability: number; // m/s²
  gyro_bias_instability: number;  // rad/s
  /** Initial uncertainty */
  initial_position_sigma: number; // meters
  initial_velocity_sigma: number; // m/s
  initial_attitude_sigma: number; // radians
  initial_accel_bias_sigma: number;
  initial_gyro_bias_sigma: number;
}

export const DEFAULT_ESKF_CONFIG: ESKFConfig = {
  accel_noise_density: 0.012,
  gyro_noise_density: 0.0087,
  accel_bias_instability: 0.001,
  gyro_bias_instability: 0.0001,
  initial_position_sigma: 10.0,
  initial_velocity_sigma: 1.0,
  initial_attitude_sigma: 0.1,
  initial_accel_bias_sigma: 0.01,
  initial_gyro_bias_sigma: 0.001,
};

/**
 * Build initial covariance matrix P₀
 */
export function buildInitialCovariance(config: ESKFConfig = DEFAULT_ESKF_CONFIG): Matrix {
  const sigmas = [
    config.initial_position_sigma, config.initial_position_sigma, config.initial_position_sigma,
    config.initial_velocity_sigma, config.initial_velocity_sigma, config.initial_velocity_sigma,
    config.initial_attitude_sigma, config.initial_attitude_sigma, config.initial_attitude_sigma,
    config.initial_accel_bias_sigma, config.initial_accel_bias_sigma, config.initial_accel_bias_sigma,
    config.initial_gyro_bias_sigma, config.initial_gyro_bias_sigma, config.initial_gyro_bias_sigma,
  ];
  return Matrix.diag(sigmas.map(s => s * s));
}

/**
 * Build process noise matrix Q for time step dt
 * 
 * Q = diag([σ²_a·dt², σ²_a·dt², σ²_a·dt², σ²_a·dt, σ²_a·dt, σ²_a·dt,
 *           σ²_g·dt, σ²_g·dt, σ²_g·dt, σ²_ba·dt, σ²_ba·dt, σ²_ba·dt,
 *           σ²_bg·dt, σ²_bg·dt, σ²_bg·dt])
 */
export function buildProcessNoise(dt: number, config: ESKFConfig = DEFAULT_ESKF_CONFIG): Matrix {
  const sa2 = config.accel_noise_density * config.accel_noise_density;
  const sg2 = config.gyro_noise_density * config.gyro_noise_density;
  const sba2 = config.accel_bias_instability * config.accel_bias_instability;
  const sbg2 = config.gyro_bias_instability * config.gyro_bias_instability;

  return Matrix.diag([
    sa2 * dt * dt, sa2 * dt * dt, sa2 * dt * dt,  // position
    sa2 * dt, sa2 * dt, sa2 * dt,                   // velocity
    sg2 * dt, sg2 * dt, sg2 * dt,                   // attitude
    sba2 * dt, sba2 * dt, sba2 * dt,               // accel bias
    sbg2 * dt, sbg2 * dt, sbg2 * dt,               // gyro bias
  ]);
}

/**
 * Build state transition matrix F for ESKF
 * Linearized around current state for time step dt
 */
export function buildStateTransition(dt: number): Matrix {
  const F = Matrix.identity(ESKF_STATE_DIM);
  // Position depends on velocity: δp += δv * dt
  F.set(0, 3, dt);
  F.set(1, 4, dt);
  F.set(2, 5, dt);
  return F;
}

/**
 * Covariance prediction step:
 * P⁻ = F · P · Fᵀ + Q
 */
export function predictCovariance(P: Matrix, F: Matrix, Q: Matrix): Matrix {
  const FP = F.multiply(P);
  const FPFt = FP.multiply(F.transpose());
  return FPFt.add(Q);
}

/**
 * Kalman gain computation:
 * K = P⁻ · Hᵀ · (H · P⁻ · Hᵀ + R)⁻¹
 */
export function computeKalmanGain(P: Matrix, H: Matrix, R: Matrix): Matrix {
  const PHt = P.multiply(H.transpose());
  const S = H.multiply(PHt).add(R); // Innovation covariance
  const Sinv = S.inverse();
  return PHt.multiply(Sinv);
}

/**
 * Covariance update step:
 * P⁺ = (I - K · H) · P⁻
 * Joseph form for numerical stability:
 * P⁺ = (I - K·H) · P⁻ · (I - K·H)ᵀ + K · R · Kᵀ
 */
export function updateCovariance(P: Matrix, K: Matrix, H: Matrix, R: Matrix): Matrix {
  const I = Matrix.identity(P.rows);
  const IKH = I.subtract(K.multiply(H));
  const IKHt = IKH.transpose();
  const KRKt = K.multiply(R).multiply(K.transpose());
  return IKH.multiply(P).multiply(IKHt).add(KRKt);
}

/**
 * Extract position uncertainty from covariance matrix
 * Returns 1-sigma horizontal uncertainty in meters
 */
export function extractPositionUncertainty(P: Matrix): { horizontal_m: number; vertical_m: number; heading_deg: number } {
  return {
    horizontal_m: Math.sqrt(P.get(0, 0) + P.get(1, 1)),
    vertical_m: Math.sqrt(P.get(2, 2)),
    heading_deg: Math.sqrt(P.get(8, 8)) * (180 / Math.PI),
  };
}

// ═══════════════════════════════════════════════════════════
// 3. BAYESIAN TRUST FUSION — Incident Validation
// ═══════════════════════════════════════════════════════════

export interface BayesianPrior {
  /** Prior probability of truth */
  p_truth: number;
  /** Prior probability of false report */
  p_false: number;
}

export interface EvidenceSource {
  source_id: string;
  /** Likelihood ratio: P(evidence | truth) / P(evidence | false) */
  likelihood_ratio: number;
  /** Source reliability [0..1] */
  reliability: number;
  /** Temporal weight (decays with age) */
  temporal_weight: number;
  /** Spatial weight (decays with distance) */
  spatial_weight: number;
}

/**
 * Bayesian fusion for incident truth probability
 * 
 * Uses log-odds form for numerical stability:
 * log_odds(posterior) = log_odds(prior) + Σ wᵢ · log(LRᵢ)
 * 
 * where wᵢ = reliability · temporal_weight · spatial_weight
 */
export function bayesianFusion(prior: BayesianPrior, evidence: EvidenceSource[]): number {
  // Convert prior to log-odds
  let logOdds = Math.log(prior.p_truth / prior.p_false);

  // Accumulate evidence
  for (const e of evidence) {
    const weight = e.reliability * e.temporal_weight * e.spatial_weight;
    if (e.likelihood_ratio > 0 && weight > 0) {
      logOdds += weight * Math.log(e.likelihood_ratio);
    }
  }

  // Convert back to probability
  const odds = Math.exp(Math.max(-20, Math.min(20, logOdds))); // Clamp for stability
  return odds / (1 + odds);
}

/**
 * Temporal decay function for evidence freshness
 * Exponential decay with half-life
 */
export function temporalDecay(ageMs: number, halfLifeMs: number): number {
  return Math.pow(0.5, ageMs / halfLifeMs);
}

/**
 * Spatial decay function for evidence proximity
 * Gaussian decay with sigma
 */
export function spatialDecay(distanceM: number, sigmaM: number): number {
  return Math.exp(-(distanceM * distanceM) / (2 * sigmaM * sigmaM));
}

/**
 * Cross-source validation
 * Checks if multiple independent sources agree
 */
export function crossSourceValidation(
  sources: { source_id: string; value: number; reliability: number }[],
  agreementThreshold: number = 0.2
): { consensus: boolean; consensusValue: number; agreementRatio: number } {
  if (sources.length === 0) return { consensus: false, consensusValue: 0, agreementRatio: 0 };

  // Weighted mean
  let weightedSum = 0;
  let totalWeight = 0;
  for (const s of sources) {
    weightedSum += s.value * s.reliability;
    totalWeight += s.reliability;
  }
  const consensusValue = totalWeight > 0 ? weightedSum / totalWeight : 0;

  // Count agreements
  let agreements = 0;
  for (const s of sources) {
    if (Math.abs(s.value - consensusValue) <= agreementThreshold) {
      agreements++;
    }
  }

  const agreementRatio = agreements / sources.length;
  return {
    consensus: agreementRatio >= 0.6,
    consensusValue,
    agreementRatio,
  };
}

// ═══════════════════════════════════════════════════════════
// 4. MULTI-OBJECTIVE ROUTE OPTIMIZATION
// ═══════════════════════════════════════════════════════════

export interface RouteObjective {
  name: string;
  weight: number;
  /** Lower is better */
  direction: 'minimize' | 'maximize';
}

export interface RouteCandidate {
  route_id: string;
  /** Objective values (same order as objectives) */
  values: number[];
}

/**
 * Multi-objective route scoring using weighted sum
 * 
 * Score = Σ wᵢ · normalize(vᵢ)
 * 
 * Normalization: (v - min) / (max - min) for minimize
 *                (max - v) / (max - min) for maximize
 */
export function scoreRoutes(
  candidates: RouteCandidate[],
  objectives: RouteObjective[]
): { route_id: string; score: number; breakdown: Record<string, number> }[] {
  if (candidates.length === 0) return [];

  // Find min/max for each objective
  const ranges = objectives.map((_, i) => {
    const values = candidates.map(c => c.values[i]);
    return { min: Math.min(...values), max: Math.max(...values) };
  });

  return candidates.map(candidate => {
    let totalScore = 0;
    const breakdown: Record<string, number> = {};

    for (let i = 0; i < objectives.length; i++) {
      const obj = objectives[i];
      const range = ranges[i];
      const span = range.max - range.min;

      let normalized: number;
      if (span === 0) {
        normalized = 0.5; // All equal
      } else if (obj.direction === 'minimize') {
        normalized = 1 - (candidate.values[i] - range.min) / span;
      } else {
        normalized = (candidate.values[i] - range.min) / span;
      }

      const weighted = normalized * obj.weight;
      totalScore += weighted;
      breakdown[obj.name] = normalized;
    }

    return { route_id: candidate.route_id, score: totalScore, breakdown };
  }).sort((a, b) => b.score - a.score);
}

/**
 * Pareto frontier extraction
 * Returns non-dominated solutions
 */
export function paretoFrontier(candidates: RouteCandidate[], objectives: RouteObjective[]): RouteCandidate[] {
  const dominated = new Set<number>();

  for (let i = 0; i < candidates.length; i++) {
    if (dominated.has(i)) continue;
    for (let j = 0; j < candidates.length; j++) {
      if (i === j || dominated.has(j)) continue;
      if (dominates(candidates[i], candidates[j], objectives)) {
        dominated.add(j);
      }
    }
  }

  return candidates.filter((_, i) => !dominated.has(i));
}

function dominates(a: RouteCandidate, b: RouteCandidate, objectives: RouteObjective[]): boolean {
  let strictlyBetter = false;
  for (let i = 0; i < objectives.length; i++) {
    const cmp = objectives[i].direction === 'minimize'
      ? a.values[i] - b.values[i]
      : b.values[i] - a.values[i];
    if (cmp > 0) return false; // a is worse in at least one objective
    if (cmp < 0) strictlyBetter = true;
  }
  return strictlyBetter;
}

// ═══════════════════════════════════════════════════════════
// 5. UNCERTAINTY BOUNDS & STABILITY
// ═══════════════════════════════════════════════════════════

/**
 * Protection Level computation for integrity monitoring
 * Based on RAIM (Receiver Autonomous Integrity Monitoring)
 * 
 * HPL = k_α · σ_horizontal
 * VPL = k_α · σ_vertical
 * 
 * where k_α is the multiplier for the desired integrity risk
 */
export function computeProtectionLevel(
  covarianceTrace: number,
  integrityRisk: number = 1e-7 // 10⁻⁷ per approach
): { hpl_m: number; vpl_m: number; available: boolean } {
  // k_α for Gaussian: inverse CDF
  const kAlpha = Math.sqrt(2) * inverseErfc(2 * integrityRisk);
  const sigma = Math.sqrt(covarianceTrace);

  return {
    hpl_m: kAlpha * sigma,
    vpl_m: kAlpha * sigma * 1.5, // Vertical typically 1.5x horizontal
    available: sigma < 100, // Only available if uncertainty is bounded
  };
}

/**
 * Approximate inverse complementary error function
 */
function inverseErfc(p: number): number {
  if (p >= 2) return -100;
  if (p <= 0) return 100;
  const pp = p < 1 ? p : 2 - p;
  const t = Math.sqrt(-2 * Math.log(pp / 2));
  let x = -0.70711 * ((2.30753 + t * 0.27061) / (1 + t * (0.99229 + t * 0.04481)) - t);
  for (let j = 0; j < 2; j++) {
    const err = erfc(x) - pp;
    x += err / (1.12837916709551 * Math.exp(-x * x) - x * err);
  }
  return p < 1 ? x : -x;
}

function erfc(x: number): number {
  const t = 1 / (1 + 0.5 * Math.abs(x));
  const tau = t * Math.exp(-x * x - 1.26551223 +
    t * (1.00002368 + t * (0.37409196 + t * (0.09678418 +
    t * (-0.18628806 + t * (0.27886807 + t * (-1.13520398 +
    t * (1.48851587 + t * (-0.82215223 + t * 0.17087277)))))))));
  return x >= 0 ? tau : 2 - tau;
}

/**
 * Mahalanobis distance for outlier detection
 * d² = (x - μ)ᵀ · P⁻¹ · (x - μ)
 */
export function mahalanobisDistance(
  measurement: number[],
  predicted: number[],
  covariance: Matrix
): number {
  if (measurement.length !== predicted.length) throw new Error('Dimension mismatch');
  const n = measurement.length;
  const innovation = new Matrix(n, 1, measurement.map((m, i) => m - predicted[i]));
  const Pinv = covariance.inverse();
  const d2 = innovation.transpose().multiply(Pinv).multiply(innovation);
  return Math.sqrt(d2.get(0, 0));
}

/**
 * Chi-squared test for measurement consistency
 * Returns true if the measurement is consistent with the predicted state
 */
export function chiSquaredTest(
  mahalanobisDist: number,
  degreesOfFreedom: number,
  significanceLevel: number = 0.05
): boolean {
  // Chi-squared critical values (approximate)
  const criticalValues: Record<number, Record<number, number>> = {
    1: { 0.05: 3.841, 0.01: 6.635 },
    2: { 0.05: 5.991, 0.01: 9.210 },
    3: { 0.05: 7.815, 0.01: 11.345 },
    6: { 0.05: 12.592, 0.01: 16.812 },
    9: { 0.05: 16.919, 0.01: 21.666 },
    15: { 0.05: 24.996, 0.01: 30.578 },
  };

  const sigKey = significanceLevel <= 0.01 ? 0.01 : 0.05;
  const threshold = criticalValues[degreesOfFreedom]?.[sigKey] ?? (degreesOfFreedom + 2 * Math.sqrt(2 * degreesOfFreedom));

  return (mahalanobisDist * mahalanobisDist) <= threshold;
}

/**
 * Convergence check for ESKF
 * The filter has converged when the covariance trace is stable
 */
export function checkConvergence(
  covarianceHistory: number[],
  windowSize: number = 10,
  threshold: number = 0.01
): { converged: boolean; rate: number } {
  if (covarianceHistory.length < windowSize) {
    return { converged: false, rate: 1.0 };
  }

  const recent = covarianceHistory.slice(-windowSize);
  const mean = recent.reduce((a, b) => a + b, 0) / recent.length;
  const variance = recent.reduce((a, b) => a + (b - mean) * (b - mean), 0) / recent.length;
  const cv = Math.sqrt(variance) / (mean || 1); // Coefficient of variation

  return {
    converged: cv < threshold,
    rate: cv,
  };
}
