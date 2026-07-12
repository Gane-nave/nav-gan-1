package com.gane.navigator;

import android.Manifest;
import android.content.Context;
import android.content.pm.PackageManager;
import android.location.GnssMeasurement;
import android.location.GnssMeasurementsEvent;
import android.location.GnssStatus;
import android.location.LocationManager;
import android.os.Build;
import androidx.core.app.ActivityCompat;
import com.getcapacitor.JSArray;
import com.getcapacitor.JSObject;
import com.getcapacitor.Plugin;
import com.getcapacitor.PluginCall;
import com.getcapacitor.PluginMethod;
import com.getcapacitor.annotation.CapacitorPlugin;
import com.getcapacitor.annotation.Permission;

import java.util.Collection;

/**
 * G.A.N.E GNSS Plugin — exposes raw multi-GNSS measurements to JavaScript.
 *
 * Requires Android API 24+ for GnssMeasurement API.
 * Requires devices with raw GNSS support (Pixel 4+, Xiaomi Mi 8+, Samsung S20+).
 *
 * Returns for each satellite:
 *   svid, constellationType, cn0DbHz, carrierFrequencyHz, pseudorangeRateMetersPerSecond,
 *   accumulatedDeltaRangeMeters, elevation, azimuth, usedInFix, hasEphemeris
 *
 * Events emitted to webview:
 *   'gnssMeasurements' — raw measurement epoch (1Hz typical)
 *   'gnssStatus'       — satellite fix status changes
 */
@CapacitorPlugin(
    name = "GnssPlugin",
    permissions = {
        @Permission(alias = "location", strings = {
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.ACCESS_COARSE_LOCATION
        })
    }
)
public class GnssPlugin extends Plugin {
    private LocationManager locationManager;
    private GnssMeasurementsEvent.Callback measurementCallback;
    private GnssStatus.Callback statusCallback;
    private boolean isTracking = false;

    @Override
    public void load() {
        super.load();
        locationManager = (LocationManager) getContext().getSystemService(Context.LOCATION_SERVICE);
    }

    @PluginMethod
    public void startTracking(PluginCall call) {
        if (isTracking) {
            call.resolve(new JSObject().put("alreadyTracking", true));
            return;
        }

        if (ActivityCompat.checkSelfPermission(getContext(), Manifest.permission.ACCESS_FINE_LOCATION)
                != PackageManager.PERMISSION_GRANTED) {
            call.reject("Location permission not granted");
            return;
        }

        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.N) {
            call.reject("Android 7.0+ required for raw GNSS");
            return;
        }

        measurementCallback = new GnssMeasurementsEvent.Callback() {
            @Override
            public void onGnssMeasurementsReceived(GnssMeasurementsEvent event) {
                super.onGnssMeasurementsReceived(event);
                emitMeasurements(event);
            }

            @Override
            public void onStatusChanged(int status) {
                JSObject ret = new JSObject();
                ret.put("status", status);
                notifyListeners("gnssMeasurementStatus", ret);
            }
        };

        statusCallback = new GnssStatus.Callback() {
            @Override
            public void onSatelliteStatusChanged(GnssStatus status) {
                emitSatelliteStatus(status);
            }
        };

        try {
            boolean measStarted = locationManager.registerGnssMeasurementsCallback(measurementCallback);
            locationManager.registerGnssStatusCallback(statusCallback);
            isTracking = true;

            JSObject ret = new JSObject();
            ret.put("measurementsStarted", measStarted);
            ret.put("statusCallbackRegistered", true);
            call.resolve(ret);
        } catch (SecurityException e) {
            call.reject("Security exception: " + e.getMessage());
        }
    }

    @PluginMethod
    public void stopTracking(PluginCall call) {
        if (!isTracking) { call.resolve(); return; }
        if (measurementCallback != null) locationManager.unregisterGnssMeasurementsCallback(measurementCallback);
        if (statusCallback != null) locationManager.unregisterGnssStatusCallback(statusCallback);
        isTracking = false;
        call.resolve();
    }

    @PluginMethod
    public void isAvailable(PluginCall call) {
        JSObject ret = new JSObject();
        ret.put("apiLevel", Build.VERSION.SDK_INT);
        ret.put("rawGnssSupported", Build.VERSION.SDK_INT >= Build.VERSION_CODES.N);
        ret.put("device", Build.MANUFACTURER + " " + Build.MODEL);
        call.resolve(ret);
    }

    private void emitMeasurements(GnssMeasurementsEvent event) {
        JSArray measurements = new JSArray();
        Collection<GnssMeasurement> items = event.getMeasurements();

        for (GnssMeasurement m : items) {
            JSObject obj = new JSObject();
            obj.put("svid", m.getSvid());
            obj.put("constellation", constellationName(m.getConstellationType()));
            obj.put("cn0", m.getCn0DbHz());
            obj.put("carrierFreq", m.hasCarrierFrequencyHz() ? m.getCarrierFrequencyHz() : null);
            obj.put("pseudorangeRate", m.getPseudorangeRateMetersPerSecond());
            obj.put("accumulatedDeltaRange", m.getAccumulatedDeltaRangeMeters());
            obj.put("timeOffsetNanos", m.getTimeOffsetNanos());
            obj.put("state", m.getState());
            obj.put("multipathIndicator", m.getMultipathIndicator());

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                if (m.hasReceivedSvTimeNanos()) {
                    obj.put("receivedSvTimeNanos", m.getReceivedSvTimeNanos());
                }
            }

            measurements.put(obj);
        }

        JSObject payload = new JSObject();
        payload.put("clockTimeNanos", event.getClock().getTimeNanos());
        payload.put("measurements", measurements);
        payload.put("epochT", System.currentTimeMillis());

        notifyListeners("gnssMeasurements", payload);
    }

    private void emitSatelliteStatus(GnssStatus status) {
        JSArray sats = new JSArray();
        int count = status.getSatelliteCount();

        for (int i = 0; i < count; i++) {
            JSObject obj = new JSObject();
            obj.put("svid", status.getSvid(i));
            obj.put("constellation", constellationName(status.getConstellationType(i)));
            obj.put("cn0", status.getCn0DbHz(i));
            obj.put("elevation", status.getElevationDegrees(i));
            obj.put("azimuth", status.getAzimuthDegrees(i));
            obj.put("usedInFix", status.usedInFix(i));
            obj.put("hasEphemeris", status.hasEphemerisData(i));
            obj.put("hasAlmanac", status.hasAlmanacData(i));

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                obj.put("hasCarrierFrequency", status.hasCarrierFrequencyHz(i));
                if (status.hasCarrierFrequencyHz(i)) {
                    obj.put("carrierFrequencyHz", status.getCarrierFrequencyHz(i));
                }
            }

            sats.put(obj);
        }

        JSObject payload = new JSObject();
        payload.put("satellites", sats);
        payload.put("epochT", System.currentTimeMillis());
        notifyListeners("gnssStatus", payload);
    }

    private String constellationName(int constellationType) {
        switch (constellationType) {
            case GnssStatus.CONSTELLATION_GPS:     return "GPS";
            case GnssStatus.CONSTELLATION_GLONASS: return "GLONASS";
            case GnssStatus.CONSTELLATION_GALILEO: return "Galileo";
            case GnssStatus.CONSTELLATION_BEIDOU:  return "BeiDou";
            case GnssStatus.CONSTELLATION_QZSS:    return "QZSS";
            case GnssStatus.CONSTELLATION_IRNSS:   return "NavIC";
            case GnssStatus.CONSTELLATION_SBAS:    return "SBAS";
            default: return "UNKNOWN";
        }
    }
}
