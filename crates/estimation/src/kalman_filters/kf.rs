
trait kf {
    fn predict(); // Step dynamics forward
    fn update(/* Measurement */); // Update based on measurements
    fn get_state() -> <f64> ; //getter function for receiving kf state
}

// TODO: KalmanDiagnostics for ensuring the filter is behaving correctly

struct KalmanFilter{
    // state array
    // measurements
    // other stuff
}