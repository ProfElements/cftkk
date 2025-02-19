pub struct Reader<Data: AsRef<[u8]>> {
    data: Data,
}

#[derive(Copy, Clone, Debug)]
pub enum Error {
    UnexpectedEnd,
    InvalidHeader,
    InvalidTokenKind,
}

impl<Data: AsRef<[u8]>> Reader<Data> {
    pub fn new(input: Data) -> Result<Self, Error> {
        let header_start = 0;
        let header_end = 2;

        let header_bytes = input
            .as_ref()
            .get(header_start..header_end)
            .ok_or(Error::UnexpectedEnd)?;

        let token = Token::from_bytes(header_bytes)?;

        if token != Token::U8(0x7C) {
            return Err(Error::InvalidHeader);
        }

        Ok(Self { data: input })
    }

    pub fn tokens(&self) -> Result<impl Iterator<Item = Token<'_>>, Error> {
        let first_token_bytes = self.data.as_ref().get(0..).ok_or(Error::UnexpectedEnd)?;
        let first_token = Token::from_bytes(first_token_bytes)?;

        let mut token_start = 0;
        Ok(core::iter::successors(Some(first_token), move |token| {
            token_start += token.len();
            let token_bytes = self.data.as_ref().get(token_start..)?;
            Token::from_bytes(token_bytes).ok()
        }))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TokenKind {
    I8,
    U8,
    I16,
    U16,
    U32,
    Hex8,
    F32,
    String,
}

impl TokenKind {
    pub fn from_bytes(input: &u8) -> Result<Self, Error> {
        match input {
            0 => Ok(TokenKind::I8),
            1 => Ok(TokenKind::U8),
            2 => Ok(TokenKind::I16),
            3 => Ok(TokenKind::U16),
            4 => Ok(TokenKind::U32),
            5 => Ok(TokenKind::Hex8),
            6 => Ok(TokenKind::F32),
            7 => Ok(TokenKind::String),
            _ => Err(Error::InvalidTokenKind),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token<'a> {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    U32(u32),
    Hex8(u32),
    F32(f32),
    String(&'a str),
}

impl<'a> Token<'a> {
    pub fn from_bytes(input: &'a [u8]) -> Result<Self, Error> {
        let [first, ref tail @ ..] = input else {
            return Err(Error::UnexpectedEnd);
        };

        let kind = TokenKind::from_bytes(first)?;

        let token_start = 0;

        let token_end = match kind {
            TokenKind::I8 => core::mem::size_of::<i8>(),
            TokenKind::U8 => core::mem::size_of::<u8>(),
            TokenKind::I16 => core::mem::size_of::<i16>(),
            TokenKind::U16 => core::mem::size_of::<u16>(),
            TokenKind::U32 | TokenKind::Hex8 => core::mem::size_of::<u32>(),
            TokenKind::F32 => core::mem::size_of::<f32>(),
            TokenKind::String => tail
                .iter()
                .position(|byte| *byte == 0)
                .ok_or(Error::InvalidTokenKind)?,
        };

        let token_bytes = tail
            .get(token_start..token_end)
            .ok_or(Error::UnexpectedEnd)?;
        match kind {
            TokenKind::I8 => Ok(Token::I8(i8::from_be_bytes(
                token_bytes.try_into().unwrap(),
            ))),
            TokenKind::U8 => Ok(Token::U8(u8::from_be_bytes(
                token_bytes.try_into().unwrap(),
            ))),
            TokenKind::I16 => Ok(Token::I16(i16::from_be_bytes(
                token_bytes.try_into().unwrap(),
            ))),
            TokenKind::U16 => Ok(Token::U16(u16::from_be_bytes(
                token_bytes.try_into().unwrap(),
            ))),
            TokenKind::U32 => Ok(Token::U32(u32::from_be_bytes(
                token_bytes.try_into().unwrap(),
            ))),
            TokenKind::Hex8 => Ok(Token::Hex8(u32::from_be_bytes(
                token_bytes.try_into().unwrap(),
            ))),
            TokenKind::F32 => Ok(Token::F32(f32::from_be_bytes(
                token_bytes.try_into().unwrap(),
            ))),
            TokenKind::String => Ok(Token::String(
                core::str::from_utf8(token_bytes.try_into().unwrap())
                    .map_err(|_| Error::InvalidTokenKind)?,
            )),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Token::U8(_) | Token::I8(_) => 2,
            Token::I16(_) | Token::U16(_) => 3,
            Token::U32(_) | Token::Hex8(_) | Token::F32(_) => 5,
            Token::String(str) => 2 + str.len(),
        }
    }

    pub fn inner_usize(&self) -> usize {
        match self {
            Token::U8(value) => usize::from(*value),
            Token::U16(value) => usize::from(*value),
            Token::U32(value) => usize::try_from(*value).unwrap(),
            _ => todo!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityClassHeader<'a> {
    ident: &'a str,
    should_make_instance: Option<Token<'a>>,
    runtime_crc: Option<Token<'a>>,
    contents_crc: Option<Token<'a>>,
    token_count: Option<Token<'a>>,
}

impl<'a> EntityClassHeader<'a> {
    pub fn from_tokens(tokens: &'a [Token]) -> EntityClassHeader<'a> {
        let Token::String(ident) = tokens[0] else {
            panic!()
        };

        if ident == "<noentclass>" {
            Self {
                ident,
                should_make_instance: None,
                runtime_crc: None,
                contents_crc: None,
                token_count: None,
            }
        } else {
            Self {
                ident,
                should_make_instance: Some(tokens[1]),
                runtime_crc: Some(tokens[2]),
                contents_crc: Some(tokens[3]),
                token_count: Some(tokens[4]),
            }
        }
    }

    pub fn len(&self) -> usize {
        if self.ident == "<noentclass>" {
            1
        } else {
            5
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityClass<'a> {
    header: EntityClassHeader<'a>,
    tokens: &'a [Token<'a>],
}

impl<'a> EntityClass<'a> {
    pub fn from_tokens(tokens: &'a [Token]) -> EntityClass<'a> {
        let header = EntityClassHeader::from_tokens(tokens);
        if let Some(count_tk) = header.token_count {
            let token_start = header.len();
            let token_end = token_start + count_tk.inner_usize();

            EntityClass {
                header,
                tokens: &tokens[token_start..token_end],
            }
        } else {
            EntityClass {
                header,
                tokens: &[],
            }
        }
    }

    pub fn len(&self) -> usize {
        self.header.len() + self.tokens.len()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum NodeType {}

#[derive(Copy, Clone, Debug)]
pub struct Node<'a> {
    pub kind: &'a str,
    name: &'a str,
    entity_class: EntityClass<'a>,
    pub tokens: &'a [Token<'a>],
}

const NODE_TYPE_LIST: &[&str] = &[
    "sector",
    "prop",
    "dynamic_light",
    "skybox",
    "spline",
    "trigger_box",
    "trigger_sphere",
    "trigger_plane",
    "trigger_beam",
    "refpoint",
    "collision_node",
    "simulation",
    "simulation_object",
    "condition",
    "camera",
    "light_matrix",
    "cellarray",
    "sound_emitter",
    "audiostream_emitter",
    "effect",
    "room",
    "portal",
    "sprite_batches",
    "group",
    "decalsystem",
    "overlayset",
    "containeroverlay",
    "textureoverlay",
    "stringoverlay",
    "meshoverlay",
    "textureboxoverlay",
    "navmesh",
    "joint",
    "datatable",
    "splash",
    "controller",
    "node",
    "advancednode",
    "dummy",
];

impl<'a> Node<'a> {
    pub fn from_tokens(tokens: &'a [Token]) -> Node<'a> {
        let Token::String(kind) = tokens[0] else {
            panic!()
        };
        let Token::String(name) = tokens[1] else {
            panic!()
        };

        let entity_class = EntityClass::from_tokens(&tokens[2..]);

        let node_tokens_start = 2 + entity_class.len();
        let node_tokens_end = tokens[node_tokens_start..]
            .iter()
            .position(|token| {
                let Token::String(name) = token else {
                    return false;
                };

                NODE_TYPE_LIST.contains(name)
            })
            .and_then(|count| Some(count + node_tokens_start))
            .unwrap_or(tokens.len() - 1);

        Node {
            kind,
            name,
            entity_class,
            tokens: &tokens[node_tokens_start..node_tokens_end],
        }
    }

    pub fn len(&self) -> usize {
        2 + self.entity_class.len() + self.tokens.len()
    }
}
