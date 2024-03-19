pub const NON_ARRAY_OR_OBJECT_EVENT: &str = r#""channel-message""#;
pub const OBJECT_INVALID_EVENT: &str = r#"{"event":"emergency", "reqid": 42}"#;
pub const OBJECT_MISSING_EVENT: &str = r#"{"reqid": 42}"#;
pub const ARRAY_INVALID_EVENT: &str = r#"[341,[],"incorrect-event","XBT/USD"]"#;
pub const ARRAY_MISSING_EVENT: &str = r#"[341,[],42,"XBT/USD"]"#;
pub const INVALID_CHANNEL_NAME: &str =
    r#"{"connectionID":7858587364768643506,"event":"airdrop","status":"online","version":"1.9.1"}"#;
pub const ARRAY_INVALID_LENGTH_FOR_EVENT: &str = r#"["channel-message"]"#;
pub const INVALID_SYSTEM_STATUS: &str =
    r#"{"connectionID":"notAnInt","event":"systemStatus","status":"online","version":"1.9.1"}"#;
pub const INVALID_UNSUBSCRIBE: &str = r#"{"channelID":341,"channelName":"spread","event":"subscriptionStatus","pair":"XBT/USD","reqid":"zero!","status":"unsubscribed","subscription":{"name":"spread"}}"#;

pub const HEARTBEAT: &str = r#"{"event":"heartbeat"}"#;
pub const PING: &str = r#"{"event":"ping", "reqid": 42}"#;
pub const PONG: &str = r#"{"event":"pong", "reqid": 42}"#;
pub const SYSTEM_STATUS: &str = r#"{"connectionID":7858587364768643506,"event":"systemStatus","status":"online","version":"1.9.1"}"#;
pub const SUBSCRIBE_SPREAD: &str = r#"{"channelID":341,"channelName":"spread","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"unsubscribed","subscription":{"name":"spread"}}"#;
pub const SPREAD: &str = r#"[341,["37080.10000","37080.20000","1699797184.943422","21.82608437","0.50775187"],"spread","XBT/USD"]"#;
pub const UNSUBSCRIBE_SPREAD: &str = r#"{"channelID":341,"channelName":"spread","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"unsubscribed","subscription":{"name":"spread"}}"#;
pub const SUBSCRIBE_OHLC: &str = r#"{"channelID":343,"channelName":"ohlc-1","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"subscribed","subscription":{"interval":1,"name":"ohlc"}}"#;
pub const OHLC: &str = r#"[343,["1699797181.803577","1699797240.000000","37080.20000","37080.20000","37080.20000","37080.20000","37080.20000","0.01032369",2],"ohlc-1","XBT/USD"]"#;
pub const UNSUBSCRIBE_OHLC: &str = r#"{"channelID":343,"channelName":"ohlc-1","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"unsubscribed","subscription":{"interval":1,"name":"ohlc"}}"#;
pub const SUBSCRIBE_TICKER: &str = r#"{"channelID":340,"channelName":"ticker","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"subscribed","subscription":{"name":"ticker"}}"#;
pub const TICKER: &str = r#"[340,{"a":["37080.20000",0,"0.49479977"],"b":["37080.10000",24,"24.49109974"],"c":["37080.10000","0.01268510"],"v":["537.03329406","1394.36071246"],"p":["37012.52371","37042.48940"],"t":[8495,21019],"l":["36727.30000","36658.00000"],"h":["37185.60000","37289.70000"],"o":["37139.90000","37160.10000"]},"ticker","XBT/USD"]"#;
pub const TICKER_UNSUBSCRIBE: &str = r#"{"channelID":340,"channelName":"ticker","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"unsubscribed","subscription":{"name":"ticker"}}"#;
pub const SUBSCRIBE_TRADE: &str = r#"{"channelID":337,"channelName":"trade","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"subscribed","subscription":{"name":"trade"}}"#;
pub const TRADE: &str =
    r#"[337,[["37080.10000","0.00015891","1699797222.188887","s","m",""]],"trade","XBT/USD"]"#;
pub const UNSUBSCRIBE_TRADE: &str = r#"{"channelID":337,"channelName":"trade","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"unsubscribed","subscription":{"name":"trade"}}"#;
pub const SUBSCRIBE_BOOK: &str = r#"{"channelID":336,"channelName":"book-10","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"subscribed","subscription":{"depth":10,"name":"book"}}"#;
pub const BOOK_SNAPSHOT: &str = r#"[336,{"as":[["37080.20000","0.44907155","1699797211.976902"],["37080.50000","0.01086516","1699797210.264751"],["37096.10000","0.00100000","1699797210.168531"]],"bs":[["37080.10000","24.49109974","1699797200.242011"],["37079.90000","0.08764809","1699797196.230889"],["37079.80000","0.02789714","1699797179.654731"]]},"book-10","XBT/USD"]"#;
pub const BOOK_SNAPSHOT_MISSING_FIELD: &str = r#"[336,{"as":[["37080.20000","0.44907155"],["37080.50000","0.01086516","1699797210.264751"],["37096.10000","0.00100000","1699797210.168531"]],"bs":[["37080.10000","24.49109974","1699797200.242011"],["37079.90000","0.08764809","1699797196.230889"],["37079.80000","0.02789714","1699797179.654731"]]},"book-10","XBT/USD"]"#;
pub const BOOK_BIDS_ONLY: &str = r#"[336,{"b":[["37079.40000","0.36000000","1699797212.921034"],["37080.10000","24.89569974","1699797212.921050"]],"c":"2845854188"},"book-10","XBT/USD"]"#;
pub const BOOK_BIDS_ONLY_MISSING_FIELD: &str = r#"[336,{"b":[["37079.40000","0.36000000","1699797212.921034"],["37080.10000","24.89569974"]],"c":"2845854188"},"book-10","XBT/USD"]"#;
pub const BOOK_ASKS_ONLY: &str = r#"[336,{"a":[["37109.60000","0.00000000","1699797213.027747"],["37110.40000","2.69466902","1699797200.313276","r"]],"c":"1339898949"},"book-10","XBT/USD"]"#;
pub const UNSUBSCRIBE_BOOK: &str = r#"{"channelID":336,"channelName":"book-10","event":"subscriptionStatus","pair":"XBT/USD","reqid":0,"status":"unsubscribed","subscription":{"depth":10,"name":"book"}}"#;
