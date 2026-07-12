/// Economic measurement: CAC, LTV, retention, stickiness.
#[derive(Debug, Clone)]
pub struct EconomicMetrics {
    pub customer_acquisition_cost: f64,
    pub lifetime_value: f64,
    pub monthly_retention_rate: f64,
    pub daily_active_users: u64,
    pub monthly_active_users: u64,
    pub total_monthly_cost: f64,
    pub total_monthly_revenue: f64,
}
impl EconomicMetrics {
    pub fn ltv_cac_ratio(&self) -> f64 {
        if self.customer_acquisition_cost <= 0.0 {
            f64::MAX
        } else {
            self.lifetime_value / self.customer_acquisition_cost
        }
    }
    pub fn stickiness(&self) -> f64 {
        if self.monthly_active_users == 0 {
            0.0
        } else {
            (self.daily_active_users as f64 / self.monthly_active_users as f64).clamp(0.0, 1.0)
        }
    }
    pub fn cost_per_user(&self) -> f64 {
        if self.monthly_active_users == 0 {
            0.0
        } else {
            self.total_monthly_cost / self.monthly_active_users as f64
        }
    }
    pub fn revenue_per_dau(&self) -> f64 {
        if self.daily_active_users == 0 {
            0.0
        } else {
            self.total_monthly_revenue / (self.daily_active_users as f64 * 30.0)
        }
    }
    pub fn churn_rate(&self) -> f64 {
        (1.0 - self.monthly_retention_rate).max(0.0)
    }
    pub fn health_score(&self) -> f64 {
        let l = (self.ltv_cac_ratio() / 5.0).min(1.0) * 0.3;
        let r = self.monthly_retention_rate.clamp(0.0, 1.0) * 0.3;
        let s = self.stickiness() * 0.2;
        let mg = if self.total_monthly_revenue > 0.0 {
            ((self.total_monthly_revenue - self.total_monthly_cost) / self.total_monthly_revenue)
                .clamp(0.0, 1.0)
        } else {
            0.0
        } * 0.2;
        (l + r + s + mg).clamp(0.0, 1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn s() -> EconomicMetrics {
        EconomicMetrics {
            customer_acquisition_cost: 5.0,
            lifetime_value: 25.0,
            monthly_retention_rate: 0.95,
            daily_active_users: 10000,
            monthly_active_users: 30000,
            total_monthly_cost: 50000.0,
            total_monthly_revenue: 100000.0,
        }
    }
    #[test]
    fn test_ltv() {
        assert!((s().ltv_cac_ratio() - 5.0).abs() < 0.01);
    }
    #[test]
    fn test_sticky() {
        assert!(s().stickiness() > 0.0 && s().stickiness() <= 1.0);
    }
    #[test]
    fn test_health() {
        assert!(s().health_score() > 0.0 && s().health_score() <= 1.0);
    }
    #[test]
    fn test_zero() {
        let m = EconomicMetrics {
            customer_acquisition_cost: 5.0,
            lifetime_value: 25.0,
            monthly_retention_rate: 0.95,
            daily_active_users: 0,
            monthly_active_users: 0,
            total_monthly_cost: 0.0,
            total_monthly_revenue: 0.0,
        };
        assert_eq!(m.stickiness(), 0.0);
    }
}
