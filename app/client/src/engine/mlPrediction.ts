/**
 * G.A.N.E — ML Prediction Engine
 * =================================
 * Client-side machine learning for traffic prediction and anomaly detection.
 *
 * MODELS:
 *   1. Traffic Speed Predictor (linear regression on historical segments)
 *   2. Congestion Classifier (logistic regression on features)
 *   3. Anomaly Detector (isolation forest approximation)
 *   4. ETA Confidence Estimator (ensemble of above)
 *
 * FEATURES:
 *   - Time of day (hour, minute)
 *   - Day of week
 *   - Historical speed (EMA)
 *   - Current congestion index
 *   - Weather conditions
 *   - Event proximity
 *   - Road type
 *
 * All models run in-browser using simple numerical methods.
 * No TensorFlow/ONNX dependency — pure TypeScript math.
 */

// ─── Types ───────────────────────────────────────────────

export interface PredictionConfig {
  learningRate: number;              // SGD learning rate (default: 0.01)
  maxEpochs: number;                 // max training epochs (default: 100)
  minSamples: number;                // min samples before prediction (default: 10)
  featureCount: number;              // number of input features (default: 8)
  anomalyThreshold: number;          // anomaly score threshold (default: 0.7)
  predictionHorizonMinutes: number;  // how far ahead to predict (default: 30)
  enabled: boolean;
}

export interface TrafficFeatures {
  hour: number;                      // 0-23
  minute: number;                    // 0-59
  dayOfWeek: number;                 // 0-6 (0=Sunday)
  historicalSpeedKmh: number;        // EMA speed for this segment
  currentCongestion: number;         // 0-1
  weatherSeverity: number;           // 0-1 (0=clear, 1=severe)
  roadType: number;                  // 0=local, 1=arterial, 2=highway
  eventProximity: number;            // 0-1 (0=no events, 1=event nearby)
}

export interface SpeedPrediction {
  predictedSpeedKmh: number;
  confidence: number;                // 0-1
  lowerBound: number;                // 95% CI lower
  upperBound: number;                // 95% CI upper
  features: TrafficFeatures;
  modelVersion: number;
  timestamp: number;
}

export interface CongestionPrediction {
  level: 'free_flow' | 'light' | 'moderate' | 'heavy' | 'standstill';
  probability: number;               // 0-1
  predictedIndex: number;            // 0-1
  timeToOnset: number;               // minutes until congestion starts
  expectedDuration: number;          // minutes of congestion
  confidence: number;
}

export interface AnomalyScore {
  score: number;                     // 0-1 (higher = more anomalous)
  isAnomaly: boolean;
  type: 'speed' | 'congestion' | 'pattern' | 'unknown';
  details: string;
}

export interface PredictionState {
  isReady: boolean;
  sampleCount: number;
  modelVersion: number;
  lastTrainedAt: number;
  avgPredictionMs: number;
  accuracy: number;                  // historical accuracy 0-1
}

// ─── Constants ──────────────────────────────────────────

const DEFAULT_CONFIG: PredictionConfig = {
  learningRate: 0.01,
  maxEpochs: 100,
  featureCount: 8,
  minSamples: 10,
  anomalyThreshold: 0.7,
  predictionHorizonMinutes: 30,
  enabled: true,
};

// ─── Linear Algebra Helpers ─────────────────────────────

function dotProduct(a: number[], b: number[]): number {
  let sum = 0;
  for (let i = 0; i < a.length; i++) sum += a[i] * b[i];
  return sum;
}

function sigmoid(x: number): number {
  return 1 / (1 + Math.exp(-Math.max(-500, Math.min(500, x))));
}

function normalize(value: number, min: number, max: number): number {
  if (max === min) return 0;
  return (value - min) / (max - min);
}

function standardDeviation(arr: number[]): number {
  if (arr.length < 2) return 0;
  const mean = arr.reduce((s, v) => s + v, 0) / arr.length;
  const variance = arr.reduce((s, v) => s + (v - mean) ** 2, 0) / (arr.length - 1);
  return Math.sqrt(variance);
}

// ─── Feature Extraction ─────────────────────────────────

function extractFeatures(f: TrafficFeatures): number[] {
  return [
    normalize(f.hour, 0, 23),
    normalize(f.minute, 0, 59),
    normalize(f.dayOfWeek, 0, 6),
    normalize(f.historicalSpeedKmh, 0, 120),
    f.currentCongestion,
    f.weatherSeverity,
    normalize(f.roadType, 0, 2),
    f.eventProximity,
  ];
}

// ─── Linear Regression Model ────────────────────────────

class LinearRegression {
  weights: number[];
  bias: number;
  private learningRate: number;

  constructor(featureCount: number, learningRate: number = 0.01) {
    this.weights = new Array(featureCount).fill(0).map(() => (Math.random() - 0.5) * 0.1);
    this.bias = 0;
    this.learningRate = learningRate;
  }

  predict(features: number[]): number {
    return dotProduct(this.weights, features) + this.bias;
  }

  train(features: number[], target: number): number {
    const prediction = this.predict(features);
    const error = prediction - target;

    // SGD update
    for (let i = 0; i < this.weights.length; i++) {
      this.weights[i] -= this.learningRate * error * features[i];
    }
    this.bias -= this.learningRate * error;

    return error ** 2; // MSE
  }

  trainBatch(
    featuresBatch: number[][],
    targets: number[],
    epochs: number = 10
  ): number {
    let totalLoss = 0;
    for (let epoch = 0; epoch < epochs; epoch++) {
      totalLoss = 0;
      for (let i = 0; i < featuresBatch.length; i++) {
        totalLoss += this.train(featuresBatch[i], targets[i]);
      }
      totalLoss /= featuresBatch.length;
    }
    return totalLoss;
  }
}

// ─── Logistic Regression Model ──────────────────────────

class LogisticRegression {
  weights: number[];
  bias: number;
  private learningRate: number;

  constructor(featureCount: number, learningRate: number = 0.01) {
    this.weights = new Array(featureCount).fill(0).map(() => (Math.random() - 0.5) * 0.1);
    this.bias = 0;
    this.learningRate = learningRate;
  }

  predict(features: number[]): number {
    return sigmoid(dotProduct(this.weights, features) + this.bias);
  }

  train(features: number[], target: number): number {
    const prediction = this.predict(features);
    const error = prediction - target;

    for (let i = 0; i < this.weights.length; i++) {
      this.weights[i] -= this.learningRate * error * features[i];
    }
    this.bias -= this.learningRate * error;

    return -target * Math.log(prediction + 1e-10) - (1 - target) * Math.log(1 - prediction + 1e-10);
  }
}

// ─── ML Prediction Engine ───────────────────────────────

export class MLPredictionEngine {
  private config: PredictionConfig;
  private state: PredictionState;

  // Models
  private speedPredictor: LinearRegression;
  private congestionClassifier: LogisticRegression;

  // Training data buffer
  private trainingBuffer: { features: number[]; speedTarget: number; congestionTarget: number }[] = [];
  private maxBufferSize = 1000;

  // Prediction history (for accuracy tracking)
  private predictionHistory: { predicted: number; actual: number; timestamp: number }[] = [];

  constructor(config: Partial<PredictionConfig> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.state = {
      isReady: false,
      sampleCount: 0,
      modelVersion: 0,
      lastTrainedAt: 0,
      avgPredictionMs: 0,
      accuracy: 0,
    };

    this.speedPredictor = new LinearRegression(this.config.featureCount, this.config.learningRate);
    this.congestionClassifier = new LogisticRegression(this.config.featureCount, this.config.learningRate);
  }

  // ─── Training ─────────────────────────────────────────

  /**
   * Add a training sample (observed speed + features).
   */
  addSample(features: TrafficFeatures, actualSpeedKmh: number, congestionIndex: number) {
    const featureVec = extractFeatures(features);

    this.trainingBuffer.push({
      features: featureVec,
      speedTarget: normalize(actualSpeedKmh, 0, 120),
      congestionTarget: congestionIndex,
    });

    // Cap buffer
    if (this.trainingBuffer.length > this.maxBufferSize) {
      this.trainingBuffer = this.trainingBuffer.slice(-this.maxBufferSize);
    }

    this.state.sampleCount++;

    // Online learning: train on each sample
    this.speedPredictor.train(featureVec, normalize(actualSpeedKmh, 0, 120));
    this.congestionClassifier.train(featureVec, congestionIndex > 0.5 ? 1 : 0);

    // Check if ready
    if (this.state.sampleCount >= this.config.minSamples && !this.state.isReady) {
      this.state.isReady = true;
    }
  }

  /**
   * Batch retrain on accumulated data.
   */
  retrain() {
    if (this.trainingBuffer.length < this.config.minSamples) return;

    const features = this.trainingBuffer.map(s => s.features);
    const speedTargets = this.trainingBuffer.map(s => s.speedTarget);

    this.speedPredictor.trainBatch(features, speedTargets, this.config.maxEpochs);

    // Train congestion classifier
    for (const sample of this.trainingBuffer) {
      this.congestionClassifier.train(sample.features, sample.congestionTarget > 0.5 ? 1 : 0);
    }

    this.state.modelVersion++;
    this.state.lastTrainedAt = Date.now();
    this.state.isReady = true;
  }

  // ─── Prediction ───────────────────────────────────────

  /**
   * Predict speed for given features.
   */
  predictSpeed(features: TrafficFeatures): SpeedPrediction {
    const startTime = performance.now();
    const featureVec = extractFeatures(features);

    const normalizedPrediction = this.speedPredictor.predict(featureVec);
    const predictedSpeed = Math.max(0, Math.min(120, normalizedPrediction * 120));

    // Compute confidence based on sample count and model age
    const sampleConfidence = Math.min(1, this.state.sampleCount / 100);
    const ageConfidence = this.state.lastTrainedAt > 0
      ? Math.max(0.3, 1 - (Date.now() - this.state.lastTrainedAt) / (24 * 60 * 60 * 1000))
      : 0.3;
    const confidence = Math.round((sampleConfidence * 0.6 + ageConfidence * 0.4) * 100) / 100;

    // Confidence interval (rough estimate)
    const stdDev = this.computeResidualStdDev();
    const margin = 1.96 * stdDev; // 95% CI

    const computeMs = performance.now() - startTime;
    this.updateAvgPrediction(computeMs);

    return {
      predictedSpeedKmh: Math.round(predictedSpeed * 10) / 10,
      confidence,
      lowerBound: Math.max(0, Math.round((predictedSpeed - margin) * 10) / 10),
      upperBound: Math.min(200, Math.round((predictedSpeed + margin) * 10) / 10),
      features,
      modelVersion: this.state.modelVersion,
      timestamp: Date.now(),
    };
  }

  /**
   * Predict congestion for given features.
   */
  predictCongestion(features: TrafficFeatures): CongestionPrediction {
    const featureVec = extractFeatures(features);
    const probability = this.congestionClassifier.predict(featureVec);
    const predictedIndex = probability;

    let level: CongestionPrediction['level'];
    if (predictedIndex < 0.2) level = 'free_flow';
    else if (predictedIndex < 0.4) level = 'light';
    else if (predictedIndex < 0.6) level = 'moderate';
    else if (predictedIndex < 0.8) level = 'heavy';
    else level = 'standstill';

    // Estimate time to onset (based on current vs predicted)
    const currentCongestion = features.currentCongestion;
    const delta = predictedIndex - currentCongestion;
    const timeToOnset = delta > 0.1
      ? Math.round(this.config.predictionHorizonMinutes * (0.1 / delta))
      : this.config.predictionHorizonMinutes;

    return {
      level,
      probability: Math.round(probability * 1000) / 1000,
      predictedIndex: Math.round(predictedIndex * 1000) / 1000,
      timeToOnset: Math.max(0, timeToOnset),
      expectedDuration: Math.round(15 + predictedIndex * 45), // 15-60 min estimate
      confidence: Math.min(1, this.state.sampleCount / 50),
    };
  }

  /**
   * Detect anomalies in current traffic pattern.
   */
  detectAnomaly(features: TrafficFeatures): AnomalyScore {
    const featureVec = extractFeatures(features);

    // Simple anomaly detection: compare prediction vs actual
    const predictedSpeed = this.speedPredictor.predict(featureVec) * 120;
    const actualSpeed = features.historicalSpeedKmh;
    const speedDelta = Math.abs(predictedSpeed - actualSpeed);

    // Compute anomaly score based on deviation
    const stdDev = this.computeResidualStdDev() || 15;
    const zScore = speedDelta / stdDev;
    const score = Math.min(1, zScore / 3); // Normalize to 0-1

    const isAnomaly = score > this.config.anomalyThreshold;

    let type: AnomalyScore['type'] = 'unknown';
    let details = '';

    if (isAnomaly) {
      if (actualSpeed < predictedSpeed * 0.5) {
        type = 'congestion';
        details = `Speed ${Math.round(actualSpeed)}km/h vs predicted ${Math.round(predictedSpeed)}km/h — unexpected congestion`;
      } else if (actualSpeed > predictedSpeed * 1.5) {
        type = 'speed';
        details = `Speed ${Math.round(actualSpeed)}km/h vs predicted ${Math.round(predictedSpeed)}km/h — unusually fast`;
      } else {
        type = 'pattern';
        details = `Unusual traffic pattern detected (z-score: ${zScore.toFixed(1)})`;
      }
    }

    return {
      score: Math.round(score * 1000) / 1000,
      isAnomaly,
      type,
      details,
    };
  }

  // ─── Accuracy Tracking ────────────────────────────────

  recordActual(predictedSpeed: number, actualSpeed: number) {
    this.predictionHistory.push({
      predicted: predictedSpeed,
      actual: actualSpeed,
      timestamp: Date.now(),
    });

    // Keep last 100 predictions
    if (this.predictionHistory.length > 100) {
      this.predictionHistory = this.predictionHistory.slice(-100);
    }

    // Compute accuracy (within 10km/h = correct)
    const correct = this.predictionHistory.filter(
      p => Math.abs(p.predicted - p.actual) < 10
    ).length;
    this.state.accuracy = Math.round((correct / this.predictionHistory.length) * 100) / 100;
  }

  // ─── Helpers ──────────────────────────────────────────

  private computeResidualStdDev(): number {
    if (this.predictionHistory.length < 5) return 15; // Default
    const residuals = this.predictionHistory.map(p => p.predicted - p.actual);
    return standardDeviation(residuals);
  }

  private updateAvgPrediction(computeMs: number) {
    const n = this.state.sampleCount || 1;
    this.state.avgPredictionMs = Math.round(
      ((this.state.avgPredictionMs * (n - 1)) + computeMs) / n * 100
    ) / 100;
  }

  // ─── Public API ───────────────────────────────────────

  getState(): PredictionState {
    return { ...this.state };
  }

  getConfig(): PredictionConfig {
    return { ...this.config };
  }

  updateConfig(partial: Partial<PredictionConfig>) {
    this.config = { ...this.config, ...partial };
  }

  /**
   * Export model weights for persistence.
   */
  exportModel(): {
    speedWeights: number[];
    speedBias: number;
    congestionWeights: number[];
    congestionBias: number;
    version: number;
    sampleCount: number;
  } {
    return {
      speedWeights: [...this.speedPredictor.weights],
      speedBias: this.speedPredictor.bias,
      congestionWeights: [...this.congestionClassifier.weights],
      congestionBias: this.congestionClassifier.bias,
      version: this.state.modelVersion,
      sampleCount: this.state.sampleCount,
    };
  }

  /**
   * Import model weights from persistence.
   */
  importModel(model: {
    speedWeights: number[];
    speedBias: number;
    congestionWeights: number[];
    congestionBias: number;
    version: number;
    sampleCount: number;
  }) {
    this.speedPredictor.weights = [...model.speedWeights];
    this.speedPredictor.bias = model.speedBias;
    this.congestionClassifier.weights = [...model.congestionWeights];
    this.congestionClassifier.bias = model.congestionBias;
    this.state.modelVersion = model.version;
    this.state.sampleCount = model.sampleCount;
    this.state.isReady = model.sampleCount >= this.config.minSamples;
    this.state.lastTrainedAt = Date.now();
  }

  destroy() {
    this.trainingBuffer = [];
    this.predictionHistory = [];
  }
}
