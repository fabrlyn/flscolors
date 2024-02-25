use super::{attribute::Attribute, color::Color, color_pair::ColorPair};

pub struct Colors {
    block_special: ColorPair,
    character_special: ColorPair,
    directory: ColorPair,
    directory_with_sticky: ColorPair,
    directory_without_sticky: ColorPair,
    executable: ColorPair,
    executable_set_gid: ColorPair,
    executable_set_uid: ColorPair,
    pipe: ColorPair,
    socket: ColorPair,
    symbolic_link: ColorPair,
}

impl Colors {
    fn as_sequence(&self) -> [ColorPair; 11] {
        [
            self.directory,
            self.symbolic_link,
            self.socket,
            self.pipe,
            self.executable,
            self.block_special,
            self.character_special,
            self.executable_set_uid,
            self.executable_set_gid,
            self.directory_with_sticky,
            self.directory_without_sticky,
        ]
    }

    pub fn to_string(&self) -> String {
        self.as_sequence()
            .iter()
            .flat_map(|c| [c.foreground, c.background])
            .map(|c| c.code())
            .collect()
    }
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            directory: Attribute::Directory.default_color_pair(),
            symbolic_link: Attribute::SymbolicLink.default_color_pair(),
            socket: Attribute::Socket.default_color_pair(),
            pipe: Attribute::Pipe.default_color_pair(),
            executable: Attribute::Executable.default_color_pair(),
            block_special: Attribute::BlockSpecial.default_color_pair(),
            character_special: Attribute::CharacterSpecial.default_color_pair(),
            executable_set_uid: Attribute::ExecutableSetUid.default_color_pair(),
            executable_set_gid: Attribute::ExecutableSetGid.default_color_pair(),
            directory_with_sticky: Attribute::DirectoryWithSticky.default_color_pair(),
            directory_without_sticky: Attribute::DirectoryWithoutSticky.default_color_pair(),
        }
    }
}

pub trait IntoColorsWithDefault: Sized {
    fn colors(self, colors: ColorPair) -> ColorPair;
}

impl IntoColorsWithDefault for Color {
    fn colors(self, colors: ColorPair) -> ColorPair {
        (self, colors.background).into()
    }
}

impl IntoColorsWithDefault for ColorPair {
    fn colors(self, _: ColorPair) -> ColorPair {
        self
    }
}

impl IntoColorsWithDefault for (Color, Color) {
    fn colors(self, _: ColorPair) -> ColorPair {
        (self.0, self.1).into()
    }
}

impl Colors {
    pub fn decode(input: &str) -> Option<Colors> {
        let (input, directory) = ColorPair::parse_internal(input).ok()?;
        let (input, symbolic_link) = ColorPair::parse_internal(input).ok()?;
        let (input, socket) = ColorPair::parse_internal(input).ok()?;
        let (input, pipe) = ColorPair::parse_internal(input).ok()?;
        let (input, executable) = ColorPair::parse_internal(input).ok()?;
        let (input, block_special) = ColorPair::parse_internal(input).ok()?;
        let (input, character_special) = ColorPair::parse_internal(input).ok()?;
        let (input, executable_set_uid) = ColorPair::parse_internal(input).ok()?;
        let (input, executable_set_gid) = ColorPair::parse_internal(input).ok()?;
        let (input, directory_with_sticky) = ColorPair::parse_internal(input).ok()?;
        let (input, directory_without_sticky) = ColorPair::parse_internal(input).ok()?;

        if !input.is_empty() {
            return None;
        }

        Some(Self {
            directory,
            symbolic_link,
            socket,
            pipe,
            executable,
            block_special,
            character_special,
            executable_set_uid,
            executable_set_gid,
            directory_with_sticky,
            directory_without_sticky,
        })
    }

    pub fn with_block_special<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.block_special = colors.colors(Attribute::BlockSpecial.default_color_pair());
        self
    }

    pub fn with_character_special<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.character_special = colors.colors(Attribute::CharacterSpecial.default_color_pair());
        self
    }

    pub fn with_directory<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.directory = colors.colors(Attribute::Directory.default_color_pair());
        self
    }

    pub fn with_directory_with_sticky<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.directory_with_sticky =
            colors.colors(Attribute::DirectoryWithSticky.default_color_pair());
        self
    }

    pub fn with_directory_without_sticky<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.directory_without_sticky =
            colors.colors(Attribute::DirectoryWithoutSticky.default_color_pair());
        self
    }

    pub fn with_executable<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.executable = colors.colors(Attribute::Executable.default_color_pair());
        self
    }

    pub fn with_executable_set_gid<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.executable_set_gid = colors.colors(Attribute::ExecutableSetGid.default_color_pair());
        self
    }

    pub fn with_executable_set_uid<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.executable_set_uid = colors.colors(Attribute::ExecutableSetUid.default_color_pair());
        self
    }

    pub fn with_pipe<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.pipe = colors.colors(Attribute::Pipe.default_color_pair());
        self
    }

    pub fn with_socket<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.socket = colors.colors(Attribute::Socket.default_color_pair());
        self
    }

    pub fn with_symbolic_link<C>(mut self, colors: C) -> Colors
    where
        C: IntoColorsWithDefault,
    {
        self.symbolic_link = colors.colors(Attribute::SymbolicLink.default_color_pair());
        self
    }
}
