use crate::parse::StatusParsingError;
use crate::sentences::SentenceType;
/// A list of errors that can occur during the creation of
/// a GeneralSentence
#[derive(Debug, Clone, PartialEq)]
pub enum NmeaSentenceError<'a> {
    /// Is thrown if the sentence was longer than 102 characters.
    /// The integer associated with this is the length of the sentence
    SentenceLengthError(usize),
    /// Is thrown if the chekcsum of the parsed and the calculated one do not match up.
    /// The first u8 is the parsed one, the second one the calculated one
    ChecksumError(u8, u8),
    /// Is thrown if parsing a raw sentence into a GeneralSentence fails
    GeneralParsingError,
    /// Is thrown if no parser exists for a given NMEA sentence type yet,
    /// contains the sentence type for which no parser was found
    TypeNotImplementedError(SentenceType),
    /// Is thrown if the checksum couldnt be parsed as a valid hex number,
    /// contains both supposedly hex digits as their utf8 representation
    HexParsingError(u8, u8),
    /// Is thrown if the type of a sentence could not be matched against
    /// a list of already known types
    UnkownTypeError(&'a [u8]),
    /// Is thrown if parsing the data of a sentence into a data struct fails
    DataParsingError(nom::Err<(&'a [u8], nom::error::ErrorKind)>),
    /// Is thrown if one of the status enums defined inside parse.rs is not able
    /// to be created based on the given input
    StatusParsingError(StatusParsingError),
}

impl<'a> core::fmt::Display for NmeaSentenceError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NmeaSentenceError::SentenceLengthError(len) => write!(f, "sentence too long: {}", len),
            NmeaSentenceError::ChecksumError(s1, s2) => write!(f, "invalid checksum: {:X} != {:X}", s1, s2),
            NmeaSentenceError::GeneralParsingError => f.write_str("sentence structure invalid"),
            NmeaSentenceError::TypeNotImplementedError(ty) => write!(f, "not implemented sentence type: {:?}", ty),
            NmeaSentenceError::HexParsingError(s1, s2) => write!(f, "hex checksum parsing error: {:X} {:X}", s1, s2),
            NmeaSentenceError::UnkownTypeError(ty) => write!(f, "unknown sentence type: {}", unsafe { core::str::from_utf8_unchecked(*ty) }),
            NmeaSentenceError::DataParsingError(nom::Err::Error((data, kind))) => write!(f, "parsing error: {:?} in '{:?}'", kind, data),
            NmeaSentenceError::DataParsingError(nom::Err::Failure((data, kind))) => write!(f, "unrecoverable parsing error: {:?} in '{:?}'", kind, data),
            NmeaSentenceError::DataParsingError(nom::Err::Incomplete(nom::Needed::Unknown)) => f.write_str("needed more data to complete parsing"),
            NmeaSentenceError::DataParsingError(nom::Err::Incomplete(nom::Needed::Size(sz))) => write!(f, "needed {} more bytes to complete parsing", sz),
            NmeaSentenceError::StatusParsingError(err) => write!(f, "parsing status: {:?}", err),
        }
    }
}
