//
//!
//!

// RETHINK:
// - use the unit as a type, like Metre and Kilogram? or just Length and Mass?

// Base quantities
// - https://en.wikipedia.org/wiki/International_System_of_Quantities#Base_quantities
// - https://en.wikipedia.org/wiki/Physical_quantity#Base_quantities
//
// also:

// Derived quantities
// - https://en.wikipedia.org/wiki/International_System_of_Quantities#Derived_quantities
// - https://en.wikipedia.org/wiki/Physical_quantity#General_derived_quantities

// the 7 base units
//
// External links:
// - <https://en.wikipedia.org/wiki/2019_redefinition_of_the_SI_base_units>

mod amount;
mod current;
mod intensity;
mod length;
mod mass;
mod temperature;
mod time;

pub use amount::Amount;
pub use current::Current;
pub use intensity::Intensity;
pub use length::{Distance, Height, Length};
pub use mass::Mass;
pub use temperature::Temperature;
pub use time::Time;

// derived

mod acceleration;
mod area;
mod charge;
mod density;
mod energy;
mod force;
mod frequency;
mod gfs;
mod moment;
mod momentum;
mod power;
mod pressure;
mod speed;
mod volume;

pub use acceleration::Acceleration;
pub use area::Area;
pub use charge::Charge;
pub use density::Density;
pub use energy::{Energy, Work};
pub use force::{Force, Weight};
pub use frequency::Frequency;
pub use gfs::{Gfs, GravitationalFieldStrength};
pub use moment::{Moment, Torque};
pub use momentum::Momentum;
pub use power::Power;
pub use pressure::Pressure;
pub use speed::{Speed, Velocity};
pub use volume::Volume;
