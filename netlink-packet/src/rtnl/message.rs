use crate::constants::*;
use failure::ResultExt;

use crate::{
    AddressBuffer, AddressMessage, DecodeError, Emitable, LinkBuffer, LinkMessage, Parseable,
    TcMessage,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RtnlMessage {
    NewLink(LinkMessage),
    DelLink(LinkMessage),
    GetLink(LinkMessage),
    SetLink(LinkMessage),
    NewAddress(AddressMessage),
    DelAddress(AddressMessage),
    GetAddress(AddressMessage),
    NewQueueDiscipline(TcMessage),
    DelQueueDiscipline(TcMessage),
    GetQueueDiscipline(TcMessage),
    NewTrafficClass(TcMessage),
    DelTrafficClass(TcMessage),
    GetTrafficClass(TcMessage),
    NewTrafficFilter(TcMessage),
    DelTrafficFilter(TcMessage),
    GetTrafficFilter(TcMessage),
}

impl RtnlMessage {
    pub fn is_new_link(&self) -> bool {
        if let RtnlMessage::NewLink(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_del_link(&self) -> bool {
        if let RtnlMessage::DelLink(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_get_link(&self) -> bool {
        if let RtnlMessage::GetLink(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_set_link(&self) -> bool {
        if let RtnlMessage::SetLink(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_new_address(&self) -> bool {
        if let RtnlMessage::NewAddress(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_del_address(&self) -> bool {
        if let RtnlMessage::DelAddress(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_get_address(&self) -> bool {
        if let RtnlMessage::GetAddress(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_new_qdisc(&self) -> bool {
        if let RtnlMessage::NewQueueDiscipline(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_del_qdisc(&self) -> bool {
        if let RtnlMessage::DelQueueDiscipline(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_get_qdisc(&self) -> bool {
        if let RtnlMessage::GetQueueDiscipline(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_new_class(&self) -> bool {
        if let RtnlMessage::NewTrafficClass(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_del_class(&self) -> bool {
        if let RtnlMessage::DelTrafficClass(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_get_class(&self) -> bool {
        if let RtnlMessage::GetTrafficClass(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_new_filter(&self) -> bool {
        if let RtnlMessage::NewTrafficFilter(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_del_filter(&self) -> bool {
        if let RtnlMessage::DelTrafficFilter(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_get_filter(&self) -> bool {
        if let RtnlMessage::GetTrafficFilter(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn message_type(&self) -> u16 {
        use self::RtnlMessage::*;

        match self {
            NewLink(_) => RTM_NEWLINK,
            DelLink(_) => RTM_DELLINK,
            GetLink(_) => RTM_GETLINK,
            SetLink(_) => RTM_SETLINK,
            NewAddress(_) => RTM_NEWADDR,
            DelAddress(_) => RTM_DELADDR,
            GetAddress(_) => RTM_GETADDR,
            NewQueueDiscipline(_) => RTM_NEWQDISC,
            DelQueueDiscipline(_) => RTM_DELQDISC,
            GetQueueDiscipline(_) => RTM_GETQDISC,
            NewTrafficClass(_) => RTM_NEWTCLASS,
            DelTrafficClass(_) => RTM_DELTCLASS,
            GetTrafficClass(_) => RTM_GETTCLASS,
            NewTrafficFilter(_) => RTM_NEWTFILTER,
            DelTrafficFilter(_) => RTM_DELTFILTER,
            GetTrafficFilter(_) => RTM_GETTFILTER,
        }
    }

    #[rustfmt::skip]
    pub(crate) fn parse(message_type: u16, buffer: &[u8]) -> Result<Self, DecodeError> {
        use self::RtnlMessage::*;
        let message = match message_type {
            // Link messages
            RTM_NEWLINK | RTM_GETLINK | RTM_DELLINK | RTM_SETLINK => {
                let msg: LinkMessage = LinkBuffer::new(&buffer)
                    .parse()
                    .context("invalid link message")?;
                match message_type {
                    RTM_NEWLINK => NewLink(msg),
                    RTM_GETLINK => GetLink(msg),
                    RTM_DELLINK => DelLink(msg),
                    RTM_SETLINK => SetLink(msg),
                    _ => unreachable!(),
                }
            }
            // Address messages
            RTM_NEWADDR | RTM_GETADDR | RTM_DELADDR => {
                let msg: AddressMessage = AddressBuffer::new(&buffer)
                    .parse()
                    .context("invalid address message")?;
                match message_type {
                    RTM_NEWADDR => NewAddress(msg),
                    RTM_GETADDR => GetAddress(msg),
                    RTM_DELADDR => DelAddress(msg),
                    _ => unreachable!(),
                }
            }
            // TC Messages
            RTM_NEWQDISC
            | RTM_DELQDISC
            | RTM_GETQDISC
            | RTM_NEWTCLASS
            | RTM_DELTCLASS
            | RTM_GETTCLASS
            | RTM_NEWTFILTER
            | RTM_DELTFILTER
            | RTM_GETTFILTER => {
                let msg: TcMessage = TcBuffer::new(&self.payload()).parse()?;
                match header.message_type() {
                    RTM_NEWQDISC => NewQueueDiscipline(msg),
                    RTM_DELQDISC => DelQueueDiscipline(msg),
                    RTM_GETQDISC => GetQueueDiscipline(msg),
                    RTM_NEWTCLASS => NewTrafficClass(msg),
                    RTM_DELTCLASS => DelTrafficClass(msg),
                    RTM_GETTCLASS => GetTrafficClass(msg),
                    RTM_NEWTFILTER => NewTrafficFilter(msg),
                    RTM_DELTFILTER => DelTrafficFilter(msg),
                    RTM_GETTFILTER => GetTrafficFilter(msg),
                    _ => unreachable!(),
                }
            }
            _ => return Err(format!("Unknown message type: {}", message_type).into()),
        };
        Ok(message)
    }
}

impl Emitable for RtnlMessage {
    #[rustfmt::skip]
    fn buffer_len(&self) -> usize {
        use self::RtnlMessage::*;
        match self {
            | NewLink(ref msg)
            | DelLink(ref msg)
            | GetLink(ref msg)
            | SetLink(ref msg)
            =>  msg.buffer_len(),

            | NewAddress(ref msg)
            | DelAddress(ref msg)
            | GetAddress(ref msg)
            => msg.buffer_len(),

            | NewQueueDiscipline(ref msg)
            | DelQueueDiscipline(ref msg)
            | GetQueueDiscipline(ref msg)
            | NewTrafficClass(ref msg)
            | DelTrafficClass(ref msg)
            | GetTrafficClass(ref msg)
            | NewTrafficFilter(ref msg)
            | DelTrafficFilter(ref msg)
            | GetTrafficFilter(ref msg)
            => msg.buffer_len()
        }
    }

    #[rustfmt::skip]
    fn emit(&self, buffer: &mut [u8]) {
        use self::RtnlMessage::*;
        match self {
            | NewLink(ref msg)
            | DelLink(ref msg)
            | GetLink(ref msg)
            | SetLink(ref msg)
            => msg.emit(buffer),

            | NewAddress(ref msg)
            | DelAddress(ref msg)
            | GetAddress(ref msg)
            => msg.emit(buffer),

            | NewQueueDiscipline(ref msg)
            | DelQueueDiscipline(ref msg)
            | GetQueueDiscipline(ref msg)
            | NewTrafficClass(ref msg)
            | DelTrafficClass(ref msg)
            | GetTrafficClass(ref msg)
            | NewTrafficFilter(ref msg)
            | DelTrafficFilter(ref msg)
            | GetTrafficFilter(ref msg)
            => msg.emit(buffer)
        }
    }
}
