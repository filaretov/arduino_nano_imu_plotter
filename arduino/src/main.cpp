#include <Arduino.h>
#include <Arduino_LSM9DS1.h>

typedef struct {
    float acc[3];
    float gyr[3];
    float mag[3];
} _IMU_t;

class IMU_t {
    const int channels = 9;
    const int n_bytes_float = 4;
    _IMU_t data;
    public:

    void read() {
        float x, y, z;
        while(!IMU.accelerationAvailable()) {}
        IMU.readAcceleration(x, y, z);
        data.acc[0] = x;
        data.acc[1] = -y;
        data.acc[2] = z;
        
        while(!IMU.gyroscopeAvailable()) {}
        IMU.readGyroscope(x, y, z);
        data.gyr[0] = x;
        data.gyr[1] = -y;
        data.gyr[2] = z;

        if(IMU.magneticFieldAvailable()) {
            IMU.readMagneticField(x, y, z);
            data.mag[0] = -x;
            data.mag[1] = -y;
            data.mag[2] = z;
        }
    }

    void writeSerial() {
        Serial.write((const char*)&data, channels * n_bytes_float);
    }

};


void setup() {
    Serial.begin(115200);
    IMU.begin();
    while(!Serial);
    pinMode(LED_BUILTIN, OUTPUT);
    digitalWrite(LED_BUILTIN, HIGH);
}

IMU_t imu;
void loop() {
    imu.read();
    imu.writeSerial();
}
