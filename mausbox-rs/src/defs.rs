pub struct RatBehaviour {
    // -- Gather Logic --
    gather_food_limit: u32, // If food<limit, try to gather food

    // -- Reproduce Logic --
    reproduct_age_check: bool,
    reproduct_child_check: bool,
    reproduct_minimum_age: u16,
    reproduct_delta_limit1: f32,
    reproduct_delta_limit2: f32,
    reproduct_minimum_child: u16,

    minimum_food_to_reproduct: u16, // Only reproduct if food>limit
    gift_food_to_child: u16,        // Give X food to the child when he is born

    // -- Kill Logic --
    dont_kill_compatible: bool, // Don't kill rats that are compatible with you
    kill_richest: bool,         // Kill rat with most food IF you don't have any food
    kill_oldest: bool,          // Kill oldest rat IF you don't have any food
    kill_thief: bool,           // Kill rat that stole from you
    revenge_parent: bool,
    revenge_brothers: bool,
    revenge_sons: bool,
    revenge_for_stealing: bool, // Apply revenge logic if somebody stole from your parent/brother/son.

    // -- Steal Logic --
    dont_steal_compatible: bool, // Don't steal from rats that are compatible with you
    dont_steal_family: bool,     // Don't steal from parents, brothers or sons
    steal_food_limit: u32,       // If food<limit, try to steal food
    steal_from_oldest: bool,     // Prefer stealing from the eldest
    steal_from_richest: bool,    // Prefer stealing from the richest

    // -- Help Request Logic --
    prefer_ask_compatible: bool,  // Prefer asking help to compatible rats
    help_request_self_max: u32,   // Ask for help ONLY if self.food<self_max
    help_request_others_min: u32, // Ask for help ONLY if other.food>other_min
    // -- Help Response Logic --
    always_accept_compatible: bool, // Always accept requests from compatible rats
    help_response_self_min: u32,
    help_response_others_max: u32,
}

pub enum Condition {
    food_reserved_major_than(u16),
    others_food_reserved_major_than(u16),
    trees_major_than(u16),
    others_is_family,
    others_age_major_than(u16),
    others_number_sons_major_than(u16),
    others_difference_minor_than(u16),
}

pub struct Environment {
    mutation: f32,

    food_produced_every: u8,
    food_produced_quantity: u32,
    food_produced_units: u32,

    observability_murder: f32,
    observability_theft: f32,

    /// Maximum lifetime of a Rat
    maximum_turns: u32,

    maximum_actionable_distance: u32,
}

pub struct Rat {
    gender: bool,
    behaviour: RatBehaviour,
}

pub struct Colony {
    rats: Vec<Rat>,
    env: Environment,

    food: u32,
    food_counter: u32,
}
