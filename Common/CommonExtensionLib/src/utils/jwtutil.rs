use serde::{Serialize, Deserialize};
use chrono::{DateTime, Duration, Utc, TimeZone, Local};
use chrono_tz::Tz;
// use std::time::{SystemTime, Duration, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}

const SECRET_KEY: &str = "xiaojunwu";
const AUD: &str = "gapi.tken";

// 生成token
pub fn create_tken() -> String {
    let dt_zone = Tz::Asia__Shanghai;
    let now_tm = Utc::now().with_timezone(&dt_zone); // let now_tm = Local::now();

    let now_tms = now_tm.timestamp();

    // 过期时间
    let exp_tm = now_tm + Duration::minutes(30);
    let exp_tms = exp_tm.timestamp();

    let claims = Claims {
        aud: AUD.to_string(),
        iss: AUD.to_string(),
        sub: "gapi.tk".to_string(),
        exp: exp_tms as usize,  // 2000000000 过期时间戳
        nbf: now_tms as usize,
        iat: now_tms as usize,
    };

    // let secuky = "xiaojunwu";
    let mut hd = Header::default(); hd.alg = Algorithm::HS256;

    let token = encode(
        &hd,
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    ).unwrap();

    // let exp_end = exp_tm.format("%Y-%m-%d %H:%M:%S").to_string();
    // let exp_start = now_tm.format("%Y-%m-%d %H:%M:%S").to_string();
    // println!("{0}, {1}, {2}",exp_start, exp_end , token);

    token
}

// token验证
pub fn verify_tken(tken: &String) -> (bool, String) {

    let secu_key = DecodingKey::from_secret(SECRET_KEY.as_ref());

    let mut validt = Validation::new(Algorithm::HS256);

    // 将String转换为&[_]
    validt.set_audience(&[AUD.to_string()]);  // 设置验证的Audience

    println!("tken: {0}", tken);

    let tken_data = decode::<Claims>(tken, &secu_key, &validt);
    match tken_data {
        Ok(t) => {
            let exp_tm = t.claims.exp;
            let now_tm = Local::now().timestamp() as usize;
            if exp_tm > now_tm {
                return (true, "success".to_string());
            }
            else {
                return (false, "token expired".to_string());
            }
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
    (false, "invalid token".to_string())
}