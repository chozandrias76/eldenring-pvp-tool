pub(crate) struct Group {
    pub(crate) group: String,
    pub(crate) commands: Vec<Commands>,
}

enum Commands {
    None,
    PositionStorage(Vec<PositionCommand>),
    RenderFlags(Vec<RenderFlagCommand>),
}

struct PositionCommand {
    nudge: Option<f32>,
    nudge_up: Option<String>,
    nudge_down: Option<String>,
    position: Option<String>,
    save: Option<String>,
}

struct RenderFlagCommand {
    cycle_color: Option<Vec<u8>>,
    flag: Option<String>,
    hotkey: Option<String>,
    flags: Option<Vec<String>>,
    label: Option<String>,
}

impl TryInto<Commands> for &str {
    type Error = &'static str;

    fn try_into(self) -> Result<Commands, Self::Error> {
        match self {
            "Position storage" => Ok(Commands::PositionStorage(vec![])),
            "Render flags" => Ok(Commands::RenderFlags(vec![])),
            _ => Ok(Commands::None),
        }
    }
}