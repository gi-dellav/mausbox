pub enum ReproductDeltaCheck {
    /// The delta of the two genotypes must be inside of the range
    Inside,
    /// The delta of the two genotypes must be outside of the range
    Outside,    
}
pub enum ReproductLogicCheck {
    /// Use AND logic gates to implement complex reproduction logics
    And,
    /// Use OR logic gates to implement complex reproduction logics
    Or,
}

pub struct RatBehaviour {
    // -- Gather Logic --
    
    // -- Reproduce Logic --
    reproduct_age_check: bool,
    reproduct_child_check: bool,
    reproduct_delta_check: ReproductDeltaCheck,
    reproduct_logic_gate: ReproductLogicCheck,

    reproduct_minimum_age: u16,
    reproduct_delta_limit1: f32,
    reproduct_delta_limit2: f32,
    reproduct_minimum_child: u16,

    // -- Kill Logic --
    kill_oldest: bool,
    kill_thief: bool,
    kill_killer: bool,

    // -- Steal Logic --
    steal_from_oldest: bool,
    steal_ 

    // -- Help Request Logic --
    
    // -- Help Response Logic --
}

pub struct Environment {
    crossing_over: f32,

    food_produced_every: u8,
    food_produced_quantity: u32,
    food_produced_units: u32,

    observability_murder: f32,
    observability_theft: f32,

    /// Maximum lifetime of a Rat
    maximum_turns: u32,
}

pub struct Rat {
    gender: bool,
    behaviour: RatBehaviour,
}

pub struct Colony {
    rats: Vec<Rat>,
    env: Environment,

    food: u32,
}
