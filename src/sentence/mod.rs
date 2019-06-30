pub(crate) mod parser;
mod structs;
pub(crate) use parser::*;
pub use structs::*;

#[cfg(test)]
mod sentence_tests {
    use super::*;
    use crate::fields::cardinality::*;
    use crate::fields::distance::*;
    use crate::fields::identity::*;
    use crate::fields::parameter::*;
    use crate::fields::speed::*;
    use crate::messages::*;

    use chrono::naive::{NaiveDate, NaiveTime};

    #[test]
    fn test_parse_dtm_0_lat_lon_alt() {
        let input = "$GPDTM,W84,,0.0,N,0.0,E,0.0,W84*6F\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DTMMessage {
                datum: "W84",
                sub_datum: "",
                lat: Some(Minute(0.)),
                ns: NorthSouth::North,
                lon: Some(Minute(0.)),
                ew: EastWest::East,
                alt: Some(Meter(0.)),
                ref_datum: "W84",
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_dtm_lat_lon_alt() {
        let input = "$GPDTM,999,,0.08,N,0.07,E,-47.7,W84*1B\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DTMMessage {
                datum: "999",
                sub_datum: "",
                lat: Some(Minute(0.08)),
                ns: NorthSouth::North,
                lon: Some(Minute(0.07)),
                ew: EastWest::East,
                alt: Some(Meter(-47.7)),
                ref_datum: "W84",
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_dtm_no_lat_lon_alt() {
        let input = "$GPDTM,999,,,N,,E,,W84*23\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::DTM(DTMMessage {
                datum: "999",
                sub_datum: "",
                lat: None,
                ns: NorthSouth::North,
                lon: None,
                ew: EastWest::East,
                alt: None,
                ref_datum: "W84",
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gbq() {
        let input = "$UPGBQ,RMC*21\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::MicroprocessorController,
            message: Message::GBQ(GBQMessage { msg_id: "RMC" }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gga() {
        let input = "$GPGGA,092725.00,4717.11399,N,00833.91590,E,1,08,1.01,499.6,M,48.0,M,,*5B\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GGA(GGAMessage {
                time: Some(NaiveTime::from_hms_milli(09, 27, 25, 00)),
                lat: Some(Degree(47.1711399)),
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391590)),
                ew: EastWest::East,
                quality: Fix::AutonomousGNSSFix,
                num_sv: Some(8),
                hdop: Some(1.01),
                alt: Some(Meter(499.6)),
                sep: Some(Meter(48.)),
                diff_age: None,
                diff_station: None,
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gsa() {
        let input = "$GNGSA,A,3,80,71,73,79,69,,,,,,,,1.83,1.09,1.47*17\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPSGLONASS,
            message: Message::GSA(GSAMessage {
                op_mode: OperationMode::Automatic,
                nav_mode: NavigationMode::Fix3D,
                sattelite_ids: [
                    Some(80),
                    Some(71),
                    Some(73),
                    Some(79),
                    Some(69),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                pdop: Some(Meter(1.83)),
                hdop: Some(Meter(1.09)),
                vdop: Some(Meter(1.47)),
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gsv() {
        let input = "$GPGSV,3,1,11,03,03,111,00,04,15,270,00,06,01,010,00,13,06,292,00*74\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GSV(GSVMessage {
                total_msgs: 3,
                msg_num: 1,
                satellite_num: 11,
                satellites: vec![
                    SatelliteInView {
                        id: Some(3),
                        elv: Some(3),
                        az: Some(111),
                        cno: Some(0),
                    },
                    SatelliteInView {
                        id: Some(4),
                        elv: Some(15),
                        az: Some(270),
                        cno: Some(0),
                    },
                    SatelliteInView {
                        id: Some(6),
                        elv: Some(1),
                        az: Some(10),
                        cno: Some(0),
                    },
                    SatelliteInView {
                        id: Some(13),
                        elv: Some(6),
                        az: Some(292),
                        cno: Some(0),
                    },
                ],
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gll() {
        let input = "$GPGLL,4717.11364,N,00833.91565,E,092321.00,A,A*60\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GLL(GLLMessage {
                lat: Some(Degree(47.171136399999995)), // floats ¯\_(ツ)_/¯
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391565)),
                ew: EastWest::East,
                time: Some(NaiveTime::from_hms(9, 23, 21)),
                status: Status::DataValid,
                pos_mode: Fix::AutonomousGNSSFix,
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_zda() {
        let input = "$GPZDA,082710.00,16,09,2002,00,00*64\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::ZDA(ZDAMessage {
                time: Some(NaiveTime::from_hms(8, 27, 10)),
                day: Some(16),
                month: Some(09),
                year: Some(2002),
                ltzh: Some(0),
                ltzn: Some(0),
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_rmc() {
        let input = "$GPRMC,083559.00,A,4717.11437,N,00833.91522,E,0.004,77.52,091202,,,A,V*2D\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::RMC(RMCMessage {
                time: NaiveTime::from_hms_opt(8, 35, 59),
                status: Status::DataValid,
                lat: Some(Degree(47.1711437)),
                ns: NorthSouth::North,
                lon: Some(Degree(8.3391522)),
                ew: EastWest::East,
                spd: Some(Knot(0.004)),
                cog: Some(Degree(77.52)),
                date: NaiveDate::from_ymd_opt(2002, 12, 09),
                mv: None,
                mv_ew: None,
                pos_mode: Fix::AutonomousGNSSFix,
                nav_status: NavigationalStatus::NotValid,
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_glq() {
        let input = "$UPGLQ,RMC*2F\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::MicroprocessorController,
            message: Message::GLQ(GLQMessage { msg_id: "RMC" }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gnq() {
        let input = "$UPGNQ,RMC*2D\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::MicroprocessorController,
            message: Message::GNQ(GNQMessage { msg_id: "RMC" }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gpq() {
        let input = "$GPGPQ,RMC*21\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GPQ(GPQMessage { msg_id: "RMC" }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gbs() {
        let input = "$GPGBS,235458.00,1.4,1.3,3.1,03,,-21.4,3.8,1,0*5A\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GBS(GBSMessage {
                time: Some(NaiveTime::from_hms(23, 54, 58)),
                lat_err: Some(Meter(1.4)),
                lon_err: Some(Meter(1.3)),
                alt_err: Some(Meter(3.1)),
                sat_prn: Some(3),
                prob: None,
                res: Some(-21.4),
                std_dev: Some(3.8),
                system_id: Some(1),
                signal_id: Some(0),
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gns() {
        let input =
            "$GNGNS,103600.01,5114.51176,N,00012.29380,W,ANNN,07,1.18,111.5,45.6,,,V*00\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPSGLONASS,
            message: Message::GNS(GNSMessage {
                time: Some(NaiveTime::from_hms_milli(10, 36, 00, 10)),
                lat: Some(Degree(51.145117600000006)), // floats ¯\_(ツ)_/¯
                ns: Some(NorthSouth::North),
                lon: Some(Degree(0.12293799999999999)), // floats ¯\_(ツ)_/¯
                ew: Some(EastWest::West),
                pos_mode: vec![Fix::AutonomousGNSSFix, Fix::NoFix, Fix::NoFix, Fix::NoFix],
                num_sv: Some(7),
                hdop: Some(1.18),
                alt: Some(Meter(111.5)),
                sep: Some(Meter(45.6)),
                diff_age: None,
                diff_station: None,
                nav_status: Status::DataInvalid,
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_grs() {
        let input = "$GNGRS,104148.00,1,2.6,2.2,-1.6,-1.1,-1.7,-1.5,5.8,1.7,,,,,1,1*52\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPSGLONASS,
            message: Message::GRS(GRSMessage {
                time: Some(NaiveTime::from_hms(10, 41, 48)),
                mode: Some(ComputationMethod::AfterGGA),
                residuals: [
                    Some(Meter(2.6)),
                    Some(Meter(2.2)),
                    Some(Meter(-1.6)),
                    Some(Meter(-1.1)),
                    Some(Meter(-1.7)),
                    Some(Meter(-1.5)),
                    Some(Meter(5.8)),
                    Some(Meter(1.7)),
                    None,
                    None,
                    None,
                    None,
                ],
                system_id: Some(1),
                signal_id: Some(1),
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_gst() {
        let input = "$GPGST,082356.00,1.8,,,,1.7,1.3,2.2*7E\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::GST(GSTMessage {
                time: Some(NaiveTime::from_hms(08, 23, 56)),
                range_rms: Some(Meter(1.8)),
                std_major: None,
                std_minor: None,
                orient: None,
                std_lat: Some(Meter(1.7)),
                std_lon: Some(Meter(1.3)),
                std_alt: Some(Meter(2.2)),
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_txt() {
        let input = "$GPTXT,01,01,02,ANTARIS ATR0620 HW 00000040*67\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::TXT(TXTMessage {
                num_msg: Some(01),
                msg_num: Some(01),
                msg_type: MessageLevel::Notice,
                text: "ANTARIS ATR0620 HW 00000040",
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_vlw() {
        let input = "$GPVLW,,N,,N,15.8,N,1.2,N*65\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::VLW(VLWMessage {
                twd: None,
                twd_unit: Some(WaterDistanceUnit::NauticalMile),
                wd: None,
                wd_unit: Some(WaterDistanceUnit::NauticalMile),
                tgd: Some(15.8),
                tgd_unit: Some(WaterDistanceUnit::NauticalMile),
                gd: Some(1.2),
                gd_unit: Some(WaterDistanceUnit::NauticalMile),
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }

    #[test]
    fn test_parse_vtg() {
        let input = "$GPVTG,77.52,T,,M,0.004,N,0.008,K,A*06\r\n";
        let expected_sentence = Sentence {
            sentence_type: SentenceType::Parametric,
            talker: Talker::GPS,
            message: Message::VTG(VTGMessage {
                cogt: Some(77.52),
                cogt_unit: Some(CourseOverGroundUnit::DegreesTrue),
                cogm: None,
                cogm_unit: Some(CourseOverGroundUnit::DegreesMagnetic),
                sogn: Some(0.004),
                sogn_unit: Some(SpeedOverGroundUnit::Knots),
                sogk: Some(0.008),
                sogk_unit: Some(SpeedOverGroundUnit::KilometersPerHour),
                pos_mode: Fix::AutonomousGNSSFix,
            }),
        };

        let expected_output = Ok(("", expected_sentence));
        assert_eq!(expected_output, parse_sentence(input));
    }
}

#[cfg(test)]
mod sentence_type_tests {
    use super::*;
    use crate::fields::parameter::*;

    #[test]
    fn test_parse_parametric_parse_sentence_type() {
        let input = "$foo bar";
        let expected_output = Ok(("foo bar", SentenceType::Parametric));
        assert_eq!(expected_output, parse_sentence_type(input));
    }

    #[test]
    fn test_parse_encapsulation_parse_sentence_type() {
        let input = "!foo bar";
        let expected_output = Ok(("foo bar", SentenceType::Encapsulation));
        assert_eq!(expected_output, parse_sentence_type(input));
    }

    #[test]
    fn test_parse_encapsulation_invalid_type() {
        let input = "*foo bar";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::OneOf)));
        assert_eq!(expected_output, parse_sentence_type(input));
    }

    #[test]
    fn test_parse_encapsulation_empty_input() {
        let input = "";
        let expected_output = Err(nom::Err::Failure((input, nom::error::ErrorKind::Complete)));
        assert_eq!(expected_output, parse_sentence_type(input));
    }
}
