use uk_trains::{open_ldbws::RequestType, *};

// Get a token from https://realtime.nationalrail.co.uk/OpenLDBWSRegistration/
const LDB_TOKEN: &str = "ENTER_TOKEN_HERE";

fn main() {
    // Request to get depature board with details, crs is station from and filter_crs is station to.
    let board_request = open_ldbws::GetDepBoardWithDetailsRequest {
        num_rows: 3,
        crs: "LBG".to_string(), // London Bridge, lookup CRS values here: https://www.nationalrail.co.uk/stations/
        filter_crs: Some("BFR".to_string()), // London Blackfriars, can be None for no set destination.
        filter_type: None,
        time_offset: None,
        time_window: None,
    };

    let client = reqwest::blocking::Client::new();
    let res = client.post(open_ldbws::SOAP_ENDPOINT) // Soap takes post request
        .header("Content-Type", "text/xml; charset=utf-8") // Text is XML
        .header("SOAPAction", board_request.generate_soap_action_header()) // Required header to tell soap what action to do
        .body(board_request.generate_request_body(LDB_TOKEN)) // Convert built request to raw
        .send() // Send HTTP request to soap endpoint
        .unwrap();
    let response_raw = res.text().unwrap(); // Raw soap response

    // Raw response->parsed station board
    let station_board = open_ldbws::GetDepBoardWithDetailsRequest::get_station_board_result(&response_raw).unwrap();

    println!("{} depatures from {} that call at {}:", board_request.num_rows, station_board.location_name, 
        board_request.filter_crs.unwrap_or("None".to_string()));
    for service in station_board.train_services.services {
        println!("{}: EXPECTED AT: {} ({})", service.std.unwrap(), service.etd.unwrap_or("On time".to_string()), 
            service.delay_reason.unwrap_or("No delay reason".to_string()));

        for calling_point_group in service.subsequent_calling_points.calling_points_list {
            println!("  ({})", calling_point_group.calling_point.len());
            for calling_point in calling_point_group.calling_point {
                println!("  {}: {} EXPECTED AT: {}", calling_point.crs, calling_point.st, calling_point.et.unwrap_or("On time".to_string()));
            }
        }
    }
}