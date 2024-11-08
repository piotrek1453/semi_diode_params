use std::f64::consts::E;

#[derive(Debug)]
pub struct Diode {
    pub sat_current: f64,
    pub voltage: f64,
    pub temperature: f64,
    pub quality_factor: f64,
}

#[derive(Debug)]
pub enum DiodeValueError {
    SatCurrentNegative,
    QualityFactorNonPositive,
    TemperatureNonPositive,
}

impl Default for Diode {
    fn default() -> Self {
        Self {
            sat_current: 1e-14,
            voltage: 0.6,
            temperature: 273.0,
            quality_factor: 1.0,
        }
    }
}

impl Diode {
    pub fn new(
        sat_current: f64,
        voltage: f64,
        temperature: f64,
        quality_factor: f64,
    ) -> Result<Self, DiodeValueError> {
        if sat_current < 0.0 {
            return Err(DiodeValueError::SatCurrentNegative);
        }
        if quality_factor <= 0.0 {
            return Err(DiodeValueError::QualityFactorNonPositive);
        }
        if temperature <= 0.0 {
            return Err(DiodeValueError::TemperatureNonPositive);
        }

        Ok(Self {
            sat_current,
            voltage,
            temperature,
            quality_factor,
        })
    }

    pub fn current(&self) -> Result<f64, DiodeValueError> {
        self.validate_values()?;
        let exponent_term = (self.voltage * physical_constants::ELEMENTARY_CHARGE)
            / (self.quality_factor * physical_constants::BOLTZMANN_CONSTANT * self.temperature);
        let current = self.sat_current * (E.powf(exponent_term) - 1.0);
        Ok(current)
    }

    fn validate_values(&self) -> Result<(), DiodeValueError> {
        if self.sat_current < 0.0 {
            return Err(DiodeValueError::SatCurrentNegative);
        }
        if self.quality_factor <= 0.0 {
            return Err(DiodeValueError::QualityFactorNonPositive);
        }
        if self.temperature <= 0.0 {
            return Err(DiodeValueError::TemperatureNonPositive);
        }
        Ok(())
    }
}
