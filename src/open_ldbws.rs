/// Documentation: https://realtime.nationalrail.co.uk/OpenLDBWS/
/// CRS codes: https://www.nationalrail.co.uk/stations/

use yaserde::*;

/// The endpoint to be posted to
pub const SOAP_ENDPOINT: &str = "https://lite.realtime.nationalrail.co.uk/OpenLDBWS/ldb12.asmx";

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "trainServices",
    namespaces = {
        "lt8" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/types"
    },
    prefix = "lt8"
)]
pub struct ServiceItemWithCallingPointsListWrapper {
    #[yaserde(rename = "service", prefix = "lt8")]
    pub services: Vec<ServiceItemWithCallingPoints>,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "GetStationBoardResult",
    namespaces = {
        "r" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/",
        "lt4" = "http://thalesgroup.com/RTTI/2015-11-27/ldb/types",
        "lt8" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/types",
    },
    prefix = "r"
)]
pub struct StationBoardWithDetails {
    #[yaserde(rename = "generatedAt", prefix = "lt4")]
    pub generated_at: String,
    #[yaserde(rename = "locationName", prefix = "lt4")]
    pub location_name: String,
    #[yaserde(rename = "trainServices", prefix = "lt8")]
    pub train_services: ServiceItemWithCallingPointsListWrapper,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "GetDepartureBoardResponse",
    namespaces = {
        "r" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/",
    },
    prefix = "r"
)]
pub struct GetDepBoardWithDetailsResponse {
    #[yaserde(rename = "GetStationBoardResult", prefix = "r")]
    pub station_board_result: StationBoardWithDetails,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(rename = "Body", namespaces = {"r" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/"})]
pub struct SoapBodyGetDepBoardWithDetailsResponse {
    #[yaserde(rename = "GetDepBoardWithDetailsResponse", prefix = "r")]
    pub dep_board_with_details_response: GetDepBoardWithDetailsResponse,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "Envelope",
    namespaces = {
      "s" = "http://schemas.xmlsoap.org/soap/envelope/",
    },
    prefix = "s"
)]
pub struct SoapEnvelopeGetDepBoardWithDetailsResponse {
    #[yaserde(rename = "Body", prefix = "s")]
    pub body: SoapBodyGetDepBoardWithDetailsResponse,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "callingPoint",
    namespaces = {
        "lt8" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/types",
    },
    prefix = "lt8"
)]
pub struct CallingPoint {
    #[yaserde(rename = "crs", prefix = "lt8")]
    pub crs: String, // Can be ???
    #[yaserde(rename = "st", prefix = "lt8")]
    pub st: String,
    #[yaserde(rename = "et", prefix = "lt8")]
    pub et: Option<String>,
    #[yaserde(rename = "at", prefix = "lt8")]
    pub at: Option<String>,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "callingPointList",
    namespaces = {
        "lt8" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/types",
    },
    prefix = "lt8"
)]
pub struct CallingPointGroup {
    #[yaserde(rename = "callingPoint", prefix = "lt8")]
    pub calling_point: Vec<CallingPoint>,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "subsequentCallingPoints",
    namespaces = {
        "lt8" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/types",
    },
    prefix = "lt8",
)]
pub struct SubsequentCallingPointsListWrapper {
    #[yaserde(rename = "callingPointList", prefix = "lt8")]
    pub calling_points_list: Vec<CallingPointGroup>, // There can be multiple branches of calling points
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "service",
    namespaces = {
        "lt4" = "http://thalesgroup.com/RTTI/2015-11-27/ldb/types",
        "lt5" = "http://thalesgroup.com/RTTI/2016-02-16/ldb/types",
        "lt8" = "http://thalesgroup.com/RTTI/2021-11-01/ldb/types",
    },
    prefix = "lt8"
)]
pub struct ServiceItemWithCallingPoints {
    #[yaserde(rename = "sta", prefix = "lt4")]
    pub sta: Option<String>,
    #[yaserde(rename = "eta", prefix = "lt4")]
    pub eta: Option<String>,
    #[yaserde(rename = "std", prefix = "lt4")]
    pub std: Option<String>,
    #[yaserde(rename = "etd", prefix = "lt4")]
    pub etd: Option<String>,
    #[yaserde(rename = "platform", prefix = "lt4")]
    pub platform: Option<String>,
    #[yaserde(rename = "operator", prefix = "lt4")]
    pub operator: String,
    #[yaserde(rename = "operatorCode", prefix = "lt4")]
    pub operator_code: String,
    #[yaserde(rename = "serviceType", prefix = "lt4")]
    pub service_type: String,
    #[yaserde(rename = "length", prefix = "lt4")]
    pub length: Option<u32>,
    #[yaserde(rename = "isCancelled", prefix = "lt4")]
    pub is_cancelled: Option<bool>,
    #[yaserde(rename = "cancelReason", prefix = "lt4")]
    pub cancel_reason: Option<String>,
    #[yaserde(rename = "delayReason", prefix = "lt4")]
    pub delay_reason: Option<String>,
    #[yaserde(rename = "serviceID", prefix = "lt4")]
    pub service_id: String,
    // #[yaserde(rename = "origin", prefix = "lt5")]
    // pub origin: Origin,
    #[yaserde(rename = "destination", prefix = "lt5")]
    pub destination: Destination,
    #[yaserde(rename = "subsequentCallingPoints", prefix = "lt8")]
    pub subsequent_calling_points: SubsequentCallingPointsListWrapper,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "location",
    namespaces = {
        "lt4" = "http://thalesgroup.com/RTTI/2015-11-27/ldb/types"
    },
    prefix = "lt4"
)]
pub struct Location {
    #[yaserde(rename = "locationName", prefix = "lt4")]
    pub location_name: String,
}

#[derive(Debug, Default, YaDeserialize, YaSerialize)]
#[yaserde(
    rename = "destination",
    namespaces = {
        "lt4" = "http://thalesgroup.com/RTTI/2015-11-27/ldb/types",
        "lt5" = "http://thalesgroup.com/RTTI/2016-02-16/ldb/types"
    },
    prefix = "lt5"
)]
pub struct Destination {
    #[yaserde(rename = "location", prefix = "lt4")]
    pub locations: Vec<Location>,
}

pub trait RequestType {
    fn id() -> &'static str;
    fn year_version() -> &'static str;
    fn values(&self) -> Vec<(&'static str, String)>;

    /// Generate the body to be used for the HTTP post request, get a token from https://realtime.nationalrail.co.uk/OpenLDBWSRegistration/
    fn generate_request_body(&self, ldb_token: &str) -> String {
        let mut request_tag = format!("        <ldb:{}Request>\n", Self::id());
        for value in self.values() {
            let value_name = value.0;
            let value_val = value.1;
            request_tag += &format!("            <ldb:{value_name}>{value_val}</ldb:{value_name}>\n");
        }
        request_tag += &format!("        </ldb:{}Request>", Self::id());

        return format!(
            r#"
<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:typ="http://thalesgroup.com/RTTI/2013-11-28/Token/types" xmlns:ldb="http://thalesgroup.com/RTTI/2021-11-01/ldb/">
    <soap:Header>
        <typ:AccessToken>
            <typ:TokenValue>{}</typ:TokenValue>
        </typ:AccessToken>
    </soap:Header>
    <soap:Body>
{}
    </soap:Body>
</soap:Envelope>"#,
            ldb_token, request_tag
        );
    }

    /// Generate header for HTTP request with key "SOAPAction"
    fn generate_soap_action_header(&self) -> String {
        format!("http://thalesgroup.com/RTTI/{}/ldb/{}", Self::year_version(), Self::id())
    }
}

pub struct GetDepBoardWithDetailsRequest {
    pub num_rows: i32,
    pub crs: String,
    pub filter_crs: Option<String>,
    pub filter_type: Option<String>,
    pub time_offset: Option<i32>,
    pub time_window: Option<i32>,
}

impl GetDepBoardWithDetailsRequest {
    /// Fails if yaserde fails to parse raw response
    pub fn get_station_board_result(response_raw: &str) -> Result<StationBoardWithDetails, String> {
        let envelope: SoapEnvelopeGetDepBoardWithDetailsResponse = de::from_str(response_raw)?;
        let station_board_result = envelope
            .body
            .dep_board_with_details_response
            .station_board_result;
        Ok(station_board_result)
    }
}

impl RequestType for GetDepBoardWithDetailsRequest {
    fn id() -> &'static str {
        "GetDepBoardWithDetails"
    }

    fn year_version() -> &'static str {
        "2015-05-14"
    }

    fn values(&self) -> Vec<(&'static str, String)> {
        let mut v = vec![
            ("numRows", self.num_rows.to_string()),
            ("crs", self.crs.clone()),
        ];

        if let Some(ref filter_crs) = self.filter_crs {
            v.push(("filterCrs", filter_crs.clone()));
        }

        if let Some(ref filter_type) = self.filter_type {
            v.push(("filterType", filter_type.clone()));
        }

        if let Some(time_offset) = self.time_offset {
            v.push(("timeOffset", time_offset.to_string()));
        }

        if let Some(time_window) = self.time_window {
            v.push(("timeWindow", time_window.to_string()));
        }

        v
    }
}