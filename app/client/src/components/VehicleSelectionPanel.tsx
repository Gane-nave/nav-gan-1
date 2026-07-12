/**
 * Vehicle Selection Panel
 * Allows users to select vehicle type and customize dimensions for route optimization
 */

import { useState } from 'react';
import { X, Truck, ChevronRight } from 'lucide-react';
import { VEHICLE_PROFILES, getAllVehicleTypes, type VehicleType, type VehicleDimensions } from '@/lib/vehicleProfiles';
import { useLanguage } from '@/contexts/LanguageContext';

interface VehicleSelectionPanelProps {
  onClose: () => void;
  onVehicleSelect?: (type: VehicleType, dimensions?: VehicleDimensions) => void;
  selectedVehicle?: VehicleType;
}

export default function VehicleSelectionPanel({
  onClose,
  onVehicleSelect,
  selectedVehicle = 'car',
}: VehicleSelectionPanelProps) {
  const { lang } = useLanguage();
  const [selected, setSelected] = useState<VehicleType>(selectedVehicle);
  const [customDimensions, setCustomDimensions] = useState(false);
  const [dimensions, setDimensions] = useState(VEHICLE_PROFILES[selected].dimensions);

  const profile = VEHICLE_PROFILES[selected];
  const isRTL = lang === 'he';

  const handleVehicleSelect = (type: VehicleType) => {
    setSelected(type);
    setDimensions(VEHICLE_PROFILES[type].dimensions);
    setCustomDimensions(false);
  };

  const handleConfirm = () => {
    onVehicleSelect?.(selected, customDimensions ? dimensions : undefined);
    onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div
        className={`bg-background rounded-2xl shadow-2xl border border-white/10 w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col ${
          isRTL ? 'rtl' : 'ltr'
        }`}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-white/10">
          <div className="flex items-center gap-3">
            <Truck className="w-6 h-6 text-gane-cyan" />
            <h2 className="text-xl font-bold text-white">
              {lang === 'he' ? 'בחר סוג כלי רכב' : 'Select Vehicle Type'}
            </h2>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 rounded-lg flex items-center justify-center hover:bg-white/10 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6 space-y-6">
          {/* Vehicle Type Grid */}
          <div>
            <h3 className="text-sm font-semibold text-white/70 mb-4 uppercase tracking-wider">
              {lang === 'he' ? 'סוגי כלים' : 'Vehicle Types'}
            </h3>
            <div className="grid grid-cols-2 sm:grid-cols-3 gap-3">
              {getAllVehicleTypes().map((type) => {
                const p = VEHICLE_PROFILES[type];
                const isSelected = type === selected;
                return (
                  <button
                    key={type}
                    onClick={() => handleVehicleSelect(type)}
                    className={`p-4 rounded-xl border-2 transition-all ${
                      isSelected
                        ? 'border-gane-cyan bg-gane-cyan/10'
                        : 'border-white/10 hover:border-white/20 bg-white/5'
                    }`}
                  >
                    <div className="text-3xl mb-2">{p.icon}</div>
                    <div className="text-xs font-medium text-white">
                      {lang === 'he' ? p.nameHe : p.nameEn}
                    </div>
                  </button>
                );
              })}
            </div>
          </div>

          {/* Vehicle Details */}
          <div className="bg-white/5 rounded-xl p-4 border border-white/10">
            <h3 className="text-sm font-semibold text-white/70 mb-4 uppercase tracking-wider">
              {lang === 'he' ? 'פרטי הכלי' : 'Vehicle Details'}
            </h3>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <div className="text-xs text-white/50 mb-1">
                  {lang === 'he' ? 'אורך' : 'Length'}
                </div>
                <div className="text-lg font-mono font-bold text-gane-cyan">
                  {profile.dimensions.length}m
                </div>
              </div>
              <div>
                <div className="text-xs text-white/50 mb-1">
                  {lang === 'he' ? 'רוחב' : 'Width'}
                </div>
                <div className="text-lg font-mono font-bold text-gane-green">
                  {profile.dimensions.width}m
                </div>
              </div>
              <div>
                <div className="text-xs text-white/50 mb-1">
                  {lang === 'he' ? 'גובה' : 'Height'}
                </div>
                <div className="text-lg font-mono font-bold text-gane-purple">
                  {profile.dimensions.height}m
                </div>
              </div>
              <div>
                <div className="text-xs text-white/50 mb-1">
                  {lang === 'he' ? 'משקל' : 'Weight'}
                </div>
                <div className="text-lg font-mono font-bold text-gane-orange">
                  {profile.weight.loaded}kg
                </div>
              </div>
            </div>
          </div>

          {/* Restrictions */}
          <div className="bg-white/5 rounded-xl p-4 border border-white/10">
            <h3 className="text-sm font-semibold text-white/70 mb-4 uppercase tracking-wider">
              {lang === 'he' ? 'הגבלות' : 'Restrictions'}
            </h3>
            <div className="space-y-2 text-sm">
              <div className="flex justify-between">
                <span className="text-white/60">
                  {lang === 'he' ? 'שיפוע מקסימלי' : 'Max Grade'}
                </span>
                <span className="text-white font-mono">{profile.restrictions.maxGrade}%</span>
              </div>
              <div className="flex justify-between">
                <span className="text-white/60">
                  {lang === 'he' ? 'רדיוס סיבוב' : 'Turning Radius'}
                </span>
                <span className="text-white font-mono">{profile.restrictions.minTurningRadius}m</span>
              </div>
              <div className="flex justify-between">
                <span className="text-white/60">
                  {lang === 'he' ? 'הימנע מנתיבים צרים' : 'Avoid Narrow Roads'}
                </span>
                <span className="text-white">
                  {profile.restrictions.restrictions.includes('narrow_roads') ? '✓' : '—'}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-white/60">
                  {lang === 'he' ? 'הימנע מגשרים' : 'Avoid Bridges'}
                </span>
                <span className="text-white">
                  {profile.restrictions.avoidBridges ? '✓' : '—'}
                </span>
              </div>
            </div>
          </div>

          {/* Custom Dimensions Toggle */}
          <div className="bg-white/5 rounded-xl p-4 border border-white/10">
            <button
              onClick={() => setCustomDimensions(!customDimensions)}
              className="w-full flex items-center justify-between text-white hover:text-gane-cyan transition-colors"
            >
              <span className="font-medium">
                {lang === 'he' ? 'התאם ממדים' : 'Customize Dimensions'}
              </span>
              <ChevronRight className={`w-5 h-5 transition-transform ${customDimensions ? 'rotate-90' : ''}`} />
            </button>

            {customDimensions && (
              <div className="mt-4 space-y-4 pt-4 border-t border-white/10">
                {(['length', 'width', 'height'] as const).map((key) => (
                  <div key={key}>
                    <label className="text-xs text-white/60 mb-2 block capitalize">
                      {lang === 'he'
                        ? key === 'length'
                          ? 'אורך (מ)'
                          : key === 'width'
                            ? 'רוחב (מ)'
                            : 'גובה (מ)'
                        : `${key} (m)`}
                    </label>
                    <input
                      type="number"
                      step="0.1"
                      value={dimensions[key]}
                      onChange={(e) =>
                        setDimensions({
                          ...dimensions,
                          [key]: parseFloat(e.target.value) || 0,
                        })
                      }
                      className="w-full px-3 py-2 bg-white/10 border border-white/20 rounded-lg text-white placeholder-white/40 focus:outline-none focus:border-gane-cyan transition-colors"
                    />
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div className="flex gap-3 p-6 border-t border-white/10 bg-white/5">
          <button
            onClick={onClose}
            className="flex-1 px-4 py-2 rounded-lg border border-white/20 text-white hover:bg-white/10 transition-colors font-medium"
          >
            {lang === 'he' ? 'ביטול' : 'Cancel'}
          </button>
          <button
            onClick={handleConfirm}
            className="flex-1 px-4 py-2 rounded-lg bg-gane-cyan text-black hover:brightness-110 transition-all font-bold"
          >
            {lang === 'he' ? 'בחר' : 'Select'}
          </button>
        </div>
      </div>
    </div>
  );
}
