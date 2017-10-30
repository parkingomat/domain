use bytes::{BigEndian, BufMut};
use ::iana::{Opcode, Rcode};
use super::parse::{Parser, ShortParser};


//------------ Header --------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Header {
    id: u16,
    flags: [u8; 2],
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

/// # Creation and Conversion
///
impl Header {
    /// Creates a new header.
    ///
    /// The new header will have all fields as either zero or false. Thus,
    /// the opcode will be `Opcode::Query` and the response code will be
    /// `Rcode::NoError`.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(parser: &mut Parser) -> Result<Self, ShortParser> {
        Ok(Header {
            id: parser.parse_u16()?,
            flags: [parser.parse_u8()?, parser.parse_u8()?],
            qdcount: parser.parse_u16()?,
            ancount: parser.parse_u16()?,
            nscount: parser.parse_u16()?,
            arcount: parser.parse_u16()?,
        })
    }

    pub fn compose_len(&self) -> usize {
        12
    }

    pub fn compose<B: BufMut>(&self, buf: &mut B) {
        buf.put_u16::<BigEndian>(self.id);
        buf.put_slice(&self.flags);
        buf.put_u16::<BigEndian>(self.qdcount);
        buf.put_u16::<BigEndian>(self.ancount);
        buf.put_u16::<BigEndian>(self.nscount);
        buf.put_u16::<BigEndian>(self.arcount);
    }
}


/// # Field Access
///
impl Header {
    /// Returns the value of the ID field.
    ///
    /// The ID field is an identifier chosen by whoever created a query
    /// and is copied into a response. It serves to match incoming responses
    /// to their request.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Sets the value of the ID field.
    pub fn set_id(&mut self, value: u16) {
        self.id = value
    }

    /// Sets the value of the ID field to a randomly chosen number.
    pub fn set_random_id(&mut self) {
        self.set_id(::rand::random())
    }

    /// Returns whether the QR bit is set.
    ///
    /// The QR bit specifies whether this message is a query (`false`) or
    /// a response (`true`). In other words, this bit is actually stating
    /// whether the message is *not* a query.
    pub fn qr(&self) -> bool {
        self.get_bit(0, 7)
    }

    /// Sets the value of the QR bit.
    ///
    pub fn set_qr(&mut self, set: bool) {
        self.set_bit(0, 7, set)
    }

    /// Returns the value of the Opcode field.
    ///
    /// This field specifies the kind of query this message contains. See
    /// the [`Opcode`] type for more information on the possible values and
    /// their meaning. Normal queries have the variant [`Opcode::Query`]
    /// which is also the value set when creating a new header.
    ///
    /// [`Opcode`]: ../../iana/opcode/enum.Opcode.html
    /// [`Opcode::Query`]: ../../iana/opcode/enum.Opcode.html#variant.Query
    pub fn opcode(&self) -> Opcode {
        Opcode::from_int((self.flags[0] >> 3) & 0x0F)
    }

    /// Sets the value of the opcode field.
    pub fn set_opcode(&mut self, opcode: Opcode) {
        self.flags[0] = self.flags[0] & 0x87 | (opcode.to_int() << 3);
    }

    /// Returns whether the AA bit is set.
    ///
    /// Using this bit, a name server generating a response states whether
    /// it is authoritative for the requested domain name, ie., whether this
    /// response is an *authoritative answer.* The field has no meaning in 
    /// a query.
    pub fn aa(&self) -> bool {
        self.get_bit(0, 2)
    }

    /// Sets the value of the AA bit.
    pub fn set_aa(&mut self, set: bool) {
        self.set_bit(0, 2, set)
    }

    /// Returns whether the TC bit is set.
    ///
    /// The *truncation* bit is set if there was more data available then
    /// fit into the message. This is typically used when employing
    /// datagram transports such as UDP to signal to try again using a
    /// stream transport such as TCP.
    pub fn tc(&self) -> bool {
        self.get_bit(0, 1)
    }

    /// Sets the value of the TC bit.
    pub fn set_tc(&mut self, set: bool) {
        self.set_bit(0, 1, set)
    }

    /// Returns whether the RD bit is set.
    ///
    /// The *recursion desired* bit may be set in a query to ask the name
    /// server to try and recursively gather a response if it doesn’t have
    /// the data available locally. The bit’s value is copied into the
    /// response.
    pub fn rd(&self) -> bool {
        self.get_bit(0, 0)
    }

    /// Sets the value of the RD bit.
    pub fn set_rd(&mut self, set: bool) {
        self.set_bit(0, 0, set)
    }

    /// Returns whether the RA bit is set.
    ///
    /// In a response, the *recursion available* bit denotes whether the
    /// responding name server supports recursion. It has no meaning in
    /// a query.
    pub fn ra(&self) -> bool {
        self.get_bit(1, 7)
    }

    /// Sets the value of the RA bit.
    pub fn set_ra(&mut self, set: bool) {
        self.set_bit(1, 7, set)
    }

    /// Returns whether the reserved bit is set.
    ///
    /// This bit must be `false` in all queries and responses.
    pub fn z(&self) -> bool {
        self.get_bit(1, 6)
    }

    /// Sets the value of the reserved bit.
    pub fn set_z(&mut self, set: bool) {
        self.set_bit(1, 6, set)
    }

    /// Returns whether the AD bit is set.
    ///
    /// The *authentic data* bit is used by security-aware recursive name
    /// servers to indicate that it considers all RRsets in its response to
    /// be authentic.
    pub fn ad(&self) -> bool {
        self.get_bit(1, 5)
    }

    /// Sets the value of the AD bit.
    pub fn set_ad(&mut self, set: bool) {
        self.set_bit(1, 5, set)
    }

    /// Returns whether the CD bit is set.
    ///
    /// The *checking disabled* bit is used by a security-aware resolver
    /// to indicate that it does not want upstream name servers to perform
    /// verification but rather would like to verify everything itself.
    pub fn cd(&self) -> bool {
        self.get_bit(1, 4)
    }

    /// Sets the value of the CD bit.
    pub fn set_cd(&mut self, set: bool) {
        self.set_bit(1, 4, set)
    }

    /// Returns the value of the RCODE field.
    ///
    /// The *response code* is used in a response to indicate what happened
    /// when processing the query. See the [`Rcode`] type for information on
    /// possible values and their meaning.
    ///
    /// [`Rcode`]: ../../iana/rcode/enum.Rcode.html
    pub fn rcode(&self) -> Rcode {
        Rcode::from_int(self.flags[1] & 0x0F)
    }

    /// Sets the value of the RCODE field.
    pub fn set_rcode(&mut self, rcode: Rcode) {
        self.flags[1] = self.flags[1] & 0xF0 | (rcode.to_int() & 0x0F);
    }


    //--- Internal helpers

    /// Returns the value of the bit at the given position.
    ///
    /// The argument `offset` gives the byte offset of the underlying bytes
    /// slice and `bit` gives the number of the bit with the most significant
    /// bit being 7.
    fn get_bit(&self, offset: usize, bit: usize) -> bool {
        self.flags[offset] & (1 << bit) != 0
    }

    /// Sets or resets the given bit.
    fn set_bit(&mut self, offset: usize, bit: usize, set: bool) {
        if set { self.flags[offset] |= 1 << bit }
        else { self.flags[offset] &= !(1 << bit) }
    }


    //--- Count fields in regular messages

    /// Returns the value of the QDCOUNT field.
    ///
    /// This field contains the number of questions in the first
    /// section of the message, normally the question section.
    pub fn qdcount(&self) -> u16 {
        self.qdcount
    }

    /// Sets the value of the QDCOUNT field.
    pub fn set_qdcount(&mut self, value: u16) {
        self.qdcount = value
    }

    /// Increase the value of the QDCOUNT field.
    ///
    /// # Panic
    ///
    /// The method panics if incrementing the counter would lead to an
    /// overflow.
    pub fn inc_qdcount(&mut self, inc: u16) {
        self.qdcount = self.qdcount.checked_add(inc).unwrap()
    }

    /// Returns the value of the ANCOUNT field.
    ///
    /// This field contains the number of resource records in the second
    /// section of the message, normally the answer section.
    pub fn ancount(&self) -> u16 {
        self.ancount
    }

    /// Sets the value of the ANCOUNT field.
    pub fn set_ancount(&mut self, value: u16) {
        self.ancount = value
    }

    /// Increases the value of the ANCOUNT field.
    pub fn inc_ancount(&mut self, inc: u16) {
        self.ancount = self.ancount.checked_add(inc).unwrap()
    }

    /// Returns the value of the NSCOUNT field.
    ///
    /// This field contains the number of resource records in the third
    /// section of the message, normally the authority section.
    pub fn nscount(&self) -> u16 {
        self.nscount
    }

    /// Sets the value of the NSCOUNT field.
    pub fn set_nscount(&mut self, value: u16) {
        self.nscount = value
    }

    /// Increases the value of the NSCOUNT field.
    pub fn inc_nscount(&mut self, inc: u16) {
        self.nscount = self.nscount.checked_add(inc).unwrap()
    }

    /// Returns the value of the ARCOUNT field.
    ///
    /// This field contains the number of resource records in the fourth
    /// section of the message, normally the additional section.
    pub fn arcount(&self) -> u16 {
        self.arcount
    }

    /// Sets the value of the ARCOUNT field.
    pub fn set_arcount(&mut self, value: u16) {
        self.arcount = value
    }

    /// Increases the value of the ARCOUNT field.
    pub fn inc_arcount(&mut self, inc: u16) {
        self.arcount = self.arcount.checked_add(inc).unwrap()
    }


    //--- Count fields in UPDATE messages

    /// Returns the value of the ZOCOUNT field.
    ///
    /// This is the same as the `qdcount()`. It is used in UPDATE queries
    /// where the first section is the zone section.
    pub fn zocount(&self) -> u16 {
        self.qdcount()
    }

    /// Sets the value of the ZOCOUNT field.
    pub fn set_zocount(&mut self, value: u16) {
        self.set_qdcount(value)
    }

    /// Increments the value of the ZOCOUNT field.
    pub fn inc_zocount(&mut self, inc: u16) {
        self.inc_qdcount(inc)
    }

    /// Returns the value of the PRCOUNT field.
    ///
    /// This is the same as the `ancount()`. It is used in UPDATE queries
    /// where the first section is the prerequisite section.
    pub fn prcount(&self) -> u16 {
        self.ancount()
    }

    /// Sete the value of the PRCOUNT field.
    pub fn set_prcount(&mut self, value: u16) {
        self.set_ancount(value)
    }

    /// Increments the value of the PRCOUNT field,
    pub fn inc_prcount(&mut self, inc: u16) {
        self.inc_ancount(inc)
    }

    /// Returns the value of the UPCOUNT field.
    ///
    /// This is the same as the `nscount()`. It is used in UPDATE queries
    /// where the first section is the update section.
    pub fn upcount(&self) -> u16 {
        self.nscount()
    }

    /// Sets the value of the UPCOUNT field.
    pub fn set_upcount(&mut self, value: u16) {
        self.set_nscount(value)
    }

    /// Increments the value of the UPCOUNT field.
    pub fn inc_upcount(&mut self, inc: u16) {
        self.inc_nscount(inc)
    }

    /// Returns the value of the ADCOUNT field.
    ///
    /// This is the same as the `arcount()`. It is used in UPDATE queries
    /// where the first section is the additional section.
    pub fn adcount(&self) -> u16 {
        self.arcount()
    }

    /// Sets the value of the ADCOUNT field.
    pub fn set_adcount(&mut self, value: u16) {
        self.set_arcount(value)
    }

    /// Increments the value of the ADCOUNT field.
    pub fn inc_adcount(&mut self, inc: u16) {
        self.inc_arcount(inc)
    }
}

