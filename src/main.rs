use serde::Deserialize;
use constcat::concat;
use const_format::formatcp;
use regex::{Regex, RegexBuilder};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize)]
struct Entry {
    content: String,
    id: String,
    title: String,
}


#[derive(Debug, Deserialize)]
struct Feed {
    title: String,
    #[serde(default, rename = "entry")]
    entries: Vec<Entry>,
}

const TIMEZONES:&str = "ACDT|ACST|ACT|ACWDT|ACWST|ADDT|ADMT|ADT|AEDT|AEST|AFT|AHDT|AHST|AKDT|AKST|AKTST|AKTT|ALMST|ALMT|AMST|AMT|ANAST|ANAT|ANT|APT|AQTST|AQTT|ARST|ART|ASHST|ASHT|AST|AWDT|AWST|AWT|AZOMT|AZOST|AZOT|AZST|AZT|BAKST|BAKT|BDST|BDT|BEAT|BEAUT|BIOT|BMT|BNT|BORT|BOST|BOT|BRST|BRT|BST|BTT|BURT|CANT|CAPT|CAST|CAT|CAWT|CCT|CDDT|CDT|CEDT|CEMT|CEST|CET|CGST|CGT|CHADT|CHAST|CHDT|CHOST|CHOT|CIST|CKHST|CKT|CLST|CLT|CMT|COST|COT|CPT|CST|CUT|CVST|CVT|CWT|CXT|ChST|DACT|DAVT|DDUT|DFT|DMT|DUSST|DUST|EASST|EAST|EAT|ECT|EDDT|EDT|EEDT|EEST|EET|EGST|EGT|EHDT|EMT|EPT|EST|ET|EWT|FET|FFMT|FJST|FJT|FKST|FKT|FMT|FNST|FNT|FORT|FRUST|FRUT|GALT|GAMT|GBGT|GEST|GET|GFT|GHST|GILT|GIT|GMT|GST|GYT|HAA|HAC|HADT|HAE|HAP|HAR|HAST|HAT|HAY|HDT|HKST|HKT|HLV|HMT|HNA|HNC|HNE|HNP|HNR|HNT|HNY|HOVST|HOVT|HST|ICT|IDDT|IDT|IHST|IMT|IOT|IRDT|IRKST|IRKT|IRST|ISST|IST|JAVT|JCST|JDT|JMT|JST|JWST|KART|KDT|KGST|KGT|KIZST|KIZT|KMT|KOST|KRAST|KRAT|KST|KUYST|KUYT|KWAT|LHDT|LHST|LINT|LKT|LMT|LMT|LMT|LMT|LRT|LST|MADMT|MADST|MADT|MAGST|MAGT|MALST|MALT|MART|MAWT|MDDT|MDST|MDT|MEST|MET|MHT|MIST|MIT|MMT|MOST|MOT|MPT|MSD|MSK|MSM|MST|MUST|MUT|MVT|MWT|MYT|NCST|NCT|NDDT|NDT|NEGT|NEST|NET|NFT|NMT|NOVST|NOVT|NPT|NRT|NST|NT|NUT|NWT|NZDT|NZMT|NZST|OMSST|OMST|ORAST|ORAT|PDDT|PDT|PEST|PET|PETST|PETT|PGT|PHOT|PHST|PHT|PKST|PKT|PLMT|PMDT|PMMT|PMST|PMT|PNT|PONT|PPMT|PPT|PST|PT|PWT|PYST|PYT|QMT|QYZST|QYZT|RET|RMT|ROTT|SAKST|SAKT|SAMT|SAST|SBT|SCT|SDMT|SDT|SET|SGT|SHEST|SHET|SJMT|SLT|SMT|SRET|SRT|SST|STAT|SVEST|SVET|SWAT|SYOT|TAHT|TASST|TAST|TBIST|TBIT|TBMT|TFT|THA|TJT|TKT|TLT|TMT|TOST|TOT|TRST|TRT|TSAT|TVT|ULAST|ULAT|URAST|URAT|UTC|UYHST|UYST|UYT|UZST|UZT|VET|VLAST|VLAT|VOLST|VOLT|VOST|VUST|VUT|WARST|WART|WAST|WAT|WDT|WEDT|WEMT|WEST|WET|WFT|WGST|WGT|WIB|WIT|WITA|WMT|WSDT|WSST|WST|WT|XJT|YAKST|YAKT|YAPT|YDDT|YDT|YEKST|YEKST|YEKT|YEKT|YERST|YERT|YPT|YST|YWT|zzz";
const NA_TIMEZONES:&str = "pacific|eastern|mountain|central";
const ALL_TIMEZONES:&str = concat!(TIMEZONES, "|", NA_TIMEZONES);

const DAYS:&str = "monday|tuesday|wednesday|thursday|friday|saturday|sunday";
const WEEKENDS:&str = "weekends?";
const ALL_DAYS:&str = concat!(DAYS, "|", WEEKENDS);

// pattern: hh:mm(:ss) (am) (est) | hh am (est)
const TIME_PATTERN:&str = formatcp!(r"(?<hours>\d{{1,2}}):(?<minutes>\d{{2}})(?::(?<seconds>\d{{2}}))?(?:\s*(?<period>am|a\.m\.|pm|p\.m\.))?(?:\s*(?<timezone>{}))?", ALL_TIMEZONES);
const RELATIVE_TIME_PATTERN:&str = formatcp!(r"(?<relhours>\d{{1,2}})\s*(?<relperiod>am|a\.m\.|pm|p\.m\.)?\s*(?<reltimezone>{})", ALL_TIMEZONES);
const TIMEZONE_PATTERN:&str = formatcp!(r"(?<onlytimezone>\W{}\W)", ALL_TIMEZONES);
const FULL_TIME_PATTERN:&str = formatcp!(r"(?<time>(?:{})|(?:{})|(?:{}))", TIME_PATTERN, RELATIVE_TIME_PATTERN, TIMEZONE_PATTERN);

lazy_static! {
    static ref RE: Regex = RegexBuilder::new(FULL_TIME_PATTERN)
        //.case_insensitive(true)
        .build().unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client =  reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/114.0")
        .build();
    let resp = client?.get("https://old.reddit.com/r/lfg/new/.rss").send()?.text()?;
    let feed: Feed = quick_xml::de::from_str(&resp)?;
    let feed_len = feed.entries.len();
    let entries:Vec<String> = feed.entries.into_iter().filter(|entry| contains_date(&entry.content)||contains_date(&entry.title)).map(|entry| extract_date(&entry) ).flatten().collect();
    println!("{:?}", entries);
    println!("feed: {:?}, entries: {:?}", feed_len, entries.len());
    Ok(())
}

fn contains_date(entry: &str) -> bool {
    RE.is_match(entry)
}

fn extract_date(entry: &Entry) -> Option<String> {
    let entry_string = entry.title.clone() + &entry.content;
    RE.captures((entry_string).as_str()).map(|cap| cap.name("time").unwrap().as_str().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_matching_time(){
        let time1 = "11:30";
        let time2 = "11:30 pm";
        let time3 = "11:30 p.m.";
        let time4 = "11:30 gmt";
        let time5 = "11:30:25 gmt";
        
        assert!(RE.is_match(time1));
        assert!(RE.is_match(time2));
        assert!(RE.is_match(time3));
        assert!(RE.is_match(time4));
        assert!(RE.is_match(time5));

        let format = format!("extract date {} from full string", time4);
        assert!(RE.is_match(&format));

        for cap in RE.captures_iter(time1) {
            assert_eq!(cap.name("time").map(|f| f.as_str()), Some(time1));
        }
        for cap in RE.captures_iter(time2) {
            assert_eq!(cap.name("time").map(|f| f.as_str()), Some(time2));
        }
        for cap in RE.captures_iter(time3) {
            assert_eq!(cap.name("time").map(|f| f.as_str()), Some(time3));
        }
        for cap in RE.captures_iter(time4) {
            assert_eq!(cap.name("time").map(|f| f.as_str()), Some(time4));
        }
        for cap in RE.captures_iter(time5) {
            assert_eq!(cap.name("time").map(|f| f.as_str()), Some(time5));
        }
        for cap in RE.captures_iter(&format) {
            assert_eq!(cap.name("time").map(|f| f.as_str()), Some(time4));
        }
    }
}
