#![allow(non_snake_case)]
use libm;

pub struct BlackScholes {
    // Stock price
    pub S: f64,
    // Strike price
    pub K: f64,
    // Risk-free rate
    pub r: f64,
    // Time to maturity as fraction of year
    pub t: f64,
    // Volatility
    pub v: f64,
}

impl BlackScholes {
    pub fn normal_cdf(&self, x: f64) -> f64 {
        return 0.5 * (1.0 + libm::erf(x / 2.0_f64.sqrt()));
    }
    pub fn get_d1(&self) -> f64 {
        return ((self.S / self.K).ln() + (self.r + (self.v.powf(2.0)) / 2.0) * self.t)
            / (self.v * self.t.sqrt());
    }
    pub fn get_d2(&self) -> f64 {
        return self.get_d1() - self.v * self.t.sqrt();
    }
    pub fn call_price(&self) -> f64 {
        return self.normal_cdf(self.get_d1()) * self.S
            - self.normal_cdf(self.get_d2()) * self.K * (-self.r * self.t).exp();
    }
    pub fn put_price(&self) -> f64 {
        return (-self.r * self.t).exp() * self.normal_cdf(-self.get_d2()) * self.K
            - self.normal_cdf(-self.get_d1()) * self.S;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let option = BlackScholes {
            S: 100.00,        // spot price
            K: 110.00,        // strike price
            r: 0.016,         // risk-free rate 1.6%
            t: 0.08333333333, // 1 month until expiration
            v: 0.15,          // 15% volatility
        };

        println!("{:.4}", option.call_price());
        println!("{:.4}", option.put_price());
    }
}
