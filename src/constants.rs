use crate::ai::PoemType;

pub static POEM_TYPES: &[PoemType] = &[
    PoemType::Sonnet,
    PoemType::Villanelle,
    // PoemType::Haiku,
    PoemType::Ekphrastic,
    PoemType::Concrete,
    PoemType::Elegy,
    PoemType::Epigram,
    // PoemType::Limerick,
    PoemType::Ballad,
    PoemType::Epitaph,
    PoemType::Ode,
    PoemType::FreeVerse,
];

pub static POEM_PROMPTS: &[&str] = &[
    "Write a prompt for a poem about robots that compete with other robots",
    "Create poem prompt about the journey from nothing to a competitive robot designed to compete in team-based games",
    "Write poem prompt for poem about students engineering a robot for team-based competitive game",
    "Write a poem prompt about young adults learning engineering through adversity by competing in team-based competitive robot games",
    "Write a poem prompt pertaining to the themes self-education, science, robotics and friendly competition"
];
