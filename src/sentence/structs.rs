use crate::fields::identity::*;
use crate::fields::parameter::*;
use crate::messages::*;

#[derive(Debug, PartialEq)]
pub struct Sentence<'a> {
    pub sentence_type: SentenceType,
    pub talker: Talker,
    pub message: Message<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Message<'a> {
    DTM(DTMMessage<'a>),
    GBQ(GBQMessage<'a>),
    GBS(GBSMessage),
    GGA(GGAMessage),
    GLL(GLLMessage),
    GLQ(GLQMessage<'a>),
    GNQ(GNQMessage<'a>),
    GNS(GNSMessage),
    GPQ(GPQMessage<'a>),
    GRS(GRSMessage),
    GSA(GSAMessage),
    GST(GSTMessage),
    GSV(GSVMessage),
    RMC(RMCMessage),
    TXT(TXTMessage<'a>),
    VLW(VLWMessage),
    VTG(VTGMessage),
    ZDA(ZDAMessage),
}

#[derive(Debug, PartialEq)]
pub(crate) enum MessageType {
    DTM,
    GBQ,
    GBS,
    GGA,
    GLL,
    GLQ,
    GNQ,
    GNS,
    GPQ,
    GRS,
    GSA,
    GST,
    GSV,
    RMC,
    TXT,
    VLW,
    VTG,
    ZDA,
}
