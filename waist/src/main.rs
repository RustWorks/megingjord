use axum::{extract, http::header, response::IntoResponse, routing::get, routing::options, routing::post, Router};
use geojson::GeoJson;
use std::net::SocketAddr;

async fn handler_new(extract::Json(payload): extract::Json<GeoJson>) -> impl IntoResponse {
    println!("{:?}", payload);

    ([(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")], "unique_id")
}

async fn handler_get(extract::Path(_id): extract::Path<String>) -> impl IntoResponse {
    let geojson_str = r##"{
      "bbox": [
        40.288084979755546,
        44.286796656548844,
        40.08757551777291,
        44.726249781548844
      ],
      "features": [
        {
          "geometry": {
            "coordinates": [
              [
                40.206326678599986,
                44.486438837945336
              ],
              [
                40.206326678599986,
                44.48540886968361
              ],
              [
                40.20619557643072,
                44.48420724004494
              ],
              [
                40.20619557643072,
                44.48283394902932
              ],
              [
                40.20619557643072,
                44.48111733525978
                  ],
              [
                40.20619557643072,
                44.47562417119729
              ],
              [
                40.20593337133151,
                44.47184762090432
              ],
              [
                40.20554006178099,
                44.46875771611916
              ],
              [
                40.20436011943715,
                44.46343621343362
              ],
              [
                40.20265572201495,
                44.4575997266172
              ],
              [
                40.200820169176595,
                44.45347985357033
              ],
              [
                40.19819786462798,
                44.450046626031266
              ],
              [
                40.19518208901684,
                44.44764336675393
              ],
              [
                40.19229730857955,
                44.446098414361344
              ],
              [
                40.1856093910576,
                44.444038477837914
              ],
              [
                40.1806258081035,
                44.44318017095314
              ],
              [
                40.1744613964591,
                44.44300850957619
              ],
              [
                40.17052637281222,
                44.44318017095314
              ],
              [
                40.163180384894545,
                44.44592675298439
              ],
              [
                40.15924470721093,
                44.448501673638695
              ],
              [
                40.15491519825512,
                44.45262154668557
              ],
              [
                40.15058541325459,
                44.45845803350197
              ],
              [
                40.147042656572985,
                44.46412285894143
              ],
              [
                40.1411376514798,
                44.476654139459
              ],
              [
                40.138381806784835,
                44.48386391729104
              ],
              [
                40.13536337243987,
                44.492790308892594
              ],
              [
                40.13313226957529,
                44.50171670049416
              ],
              [
                40.13103234114445,
                44.51373299688088
              ],
              [
                40.13037610020372,
                44.522144404351586
              ],
              [
                40.1301136020533,
                44.52832421392189
              ],
              [
                40.130244851255235,
                44.53467568486917
              ],
              [
                40.131557329334946,
                44.54102715581643
              ],
              [
                40.134969653727474,
                44.548236933648454
              ],
              [
                40.138381806784835,
                44.552871790826195
              ],
              [
                40.144943158011245,
                44.55973824590431
              ],
              [
                40.15189749846566,
                44.566948023736344
              ],
              [
                40.1630491993141,
                44.57724770635354
              ],
              [
                40.16908347364199,
                44.581367579400414
              ],
              [
                40.1768223011295,
                44.585315791070336
              ],
              [
                40.18442910186437,
                44.586689082085954
              ],
              [
                40.19111713575886,
                44.586689082085954
              ],
              [
                40.20055794328567,
                44.58428582280862
              ],
              [
                40.20527785414617,
                44.58171090215431
              ],
              [
                40.21012853120841,
                44.57776269048439
              ],
              [
                40.213405819195685,
                44.57381447881448
              ],
              [
                40.21720727471351,
                44.56763466924416
              ],
              [
                40.22284352311338,
                44.553901759087914
              ],
              [
                40.22559593913206,
                44.54428872197853
              ],
              [
                40.229003537353975,
                44.53072747319924
              ],
              [
                40.231886755797206,
                44.51888283818948
              ],
              [
                40.234638804356294,
                44.50394829839455
              ],
              [
                40.235031945020744,
                44.49656685918556
              ],
              [
                40.23476985149805,
                44.49158867925393
              ],
              [
                40.23359041809252,
                44.48609551519142
              ],
              [
                40.23136254339161,
                44.48094567388283
              ],
              [
                40.225989132291225,
                44.47390755742775
              ],
              [
                40.22087744320301,
                44.469272700250016
              ],
              [
                40.214454517880206,
                44.46377953618752
              ],
              [
                40.20789988485327,
                44.4575997266172
              ],
              [
                40.200164602547645,
                44.45038994878517
              ]
            ],
            "type": "LineString"
          },
          "properties": {
            "color": "#ff0000",
            "width": 2
          },
          "type": "Feature"
        },
        {
          "geometry": {
            "coordinates": [
              [
                40.22389207573567,
                44.4931336316465
              ],
              [
                40.22389207573567,
                44.4931336316465
              ],
              [
                40.22389207573567,
                44.4931336316465
              ],
              [
                40.22310566279048,
                44.49347695440041
              ],
              [
                40.22074636918031,
                44.495193568169945
              ],
              [
                40.21864915032988,
                44.49673852056251
              ],
              [
                40.21537211591651,
                44.49931344121682
              ],
              [
                40.21012853120841,
                44.50291833013283
              ],
              [
                40.20226239344754,
                44.508754816949235
              ],
              [
                40.19570658136661,
                44.51253136724221
              ],
              [
                40.18744535565079,
                44.51579293340431
              ],
              [
                40.17983889311907,
                44.517681208550805
              ],
              [
                40.17314974726128,
                44.51871117681252
              ],
              [
                40.16462340954856,
                44.51888283818948
              ],
              [
                40.159638285245755,
                44.51802453130471
              ],
              [
                40.15701438858682,
                44.516994563042985
              ],
              [
                40.15504639955199,
                44.51544961065041
              ],
              [
                40.15334076292088,
                44.51390465825784
              ],
              [
                40.15294714838396,
                44.51338967412697
              ],
              [
                40.15281594303136,
                44.51304635137306
              ],
              [
                40.15281594303136,
                44.51304635137306
              ],
              [
                40.15281594303136,
                44.51304635137306
              ]
            ],
            "type": "LineString"
          },
          "properties": {
            "color": "#ff0000",
            "width": 2
          },
          "type": "Feature"
        },
        {
          "geometry": {
            "coordinates": [
              [
                40.20921086217229,
                44.550125208794945
              ],
              [
                40.20829318071141,
                44.54892357915627
              ],
              [
                40.2060644740079,
                44.54634865850198
              ],
              [
                40.203049048300365,
                44.54257210820901
              ],
              [
                40.19728003407543,
                44.53553399175392
              ],
              [
                40.1916416595479,
                44.52952584356057
              ],
              [
                40.18652737956573,
                44.52506264775978
              ],
              [
                40.18101926217315,
                44.52145775884377
              ],
              [
                40.17695346009167,
                44.51922616094338
              ],
              [
                40.168296424802136,
                44.51665124028908
              ],
              [
                40.163180384894545,
                44.516479578912126
              ],
              [
                40.15976947741706,
                44.516479578912126
              ],
              [
                40.15530880138529,
                44.51665124028908
              ],
              [
                40.15294714838396,
                44.516994563042985
              ],
              [
                40.15229111908613,
                44.51716622441994
              ],
              [
                40.15202870559263,
                44.51716622441994
              ],
              [
                40.15189749846566,
                44.51716622441994
              ],
              [
                40.15189749846566,
                44.51716622441994
              ],
              [
                40.15189749846566,
                44.51716622441994
              ]
            ],
            "type": "LineString"
          },
          "properties": {
            "color": "#ff0000",
            "width": 2
          },
          "type": "Feature"
        }
      ],
      "type": "FeatureCollection"
    }"##;

    let geojson: GeoJson = geojson_str.parse::<GeoJson>().unwrap();

    ([(header::CONTENT_TYPE, "application/geo+json")], geojson.to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "What are you doing here?" }))
        .route("/new", post(handler_new))
        .route("/new", options(handler_new))
        .route("/get/:id", get(handler_get));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}