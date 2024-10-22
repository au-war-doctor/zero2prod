import http from 'k6/http';
import {sleep} from 'k6';
 
export default function () {
    // Wait for watchtower to update prod image
    sleep(90);

    // simple test GET at health_check
    const res = http.get("http://10.10.0.6:8000/health_check");
    console.log(JSON.stringify(res.headers));
}