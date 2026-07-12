/**
 * Vehicle Profile System
 * Defines vehicle types, dimensions, and constraints for route optimization
 */

export type VehicleType = 
  | 'car' 
  | 'motorcycle' 
  | 'truck_light' 
  | 'truck_medium' 
  | 'truck_heavy' 
  | 'bus' 
  | 'emergency' 
  | 'military'
  | 'van'
  | 'bicycle'
  | 'pedestrian';

export interface VehicleDimensions {
  length: number;      // meters
  width: number;       // meters
  height: number;      // meters
  wheelbase: number;   // meters (for turning radius)
}

export interface VehicleWeight {
  empty: number;       // kg (curb weight)
  loaded: number;      // kg (max gross weight)
  axles: number;       // number of axles
}

export interface VehicleProfile {
  type: VehicleType;
  nameEn: string;
  nameHe: string;
  icon: string;
  color: string;
  dimensions: VehicleDimensions;
  weight: VehicleWeight;
  restrictions: {
    maxGrade: number;           // max slope in %
    minTurningRadius: number;   // meters
    maxRoadWidth: number;       // meters (optional)
    avoidTunnels: boolean;
    avoidBridges: boolean;
    hazmatAllowed: boolean;
    restrictions: string[];     // e.g., ['low_bridges', 'narrow_roads', 'steep_hills']
  };
  fuelType: 'gasoline' | 'diesel' | 'electric' | 'hybrid' | 'none';
  averageSpeed: number;         // km/h
}

export const VEHICLE_PROFILES: Record<VehicleType, VehicleProfile> = {
  car: {
    type: 'car',
    nameEn: 'Car',
    nameHe: 'מכונית',
    icon: '🚗',
    color: '#4488ff',
    dimensions: {
      length: 4.5,
      width: 1.8,
      height: 1.5,
      wheelbase: 2.7,
    },
    weight: {
      empty: 1200,
      loaded: 1500,
      axles: 2,
    },
    restrictions: {
      maxGrade: 25,
      minTurningRadius: 5.5,
      maxRoadWidth: 2.0,
      avoidTunnels: false,
      avoidBridges: false,
      hazmatAllowed: false,
      restrictions: [],
    },
    fuelType: 'gasoline',
    averageSpeed: 80,
  },
  motorcycle: {
    type: 'motorcycle',
    nameEn: 'Motorcycle',
    nameHe: 'אופנוע',
    icon: '🏍️',
    color: '#ff9900',
    dimensions: {
      length: 2.0,
      width: 0.8,
      height: 1.2,
      wheelbase: 1.4,
    },
    weight: {
      empty: 200,
      loaded: 300,
      axles: 2,
    },
    restrictions: {
      maxGrade: 40,
      minTurningRadius: 3.0,
      maxRoadWidth: 0.9,
      avoidTunnels: false,
      avoidBridges: false,
      hazmatAllowed: false,
      restrictions: [],
    },
    fuelType: 'gasoline',
    averageSpeed: 100,
  },
  truck_light: {
    type: 'truck_light',
    nameEn: 'Light Truck',
    nameHe: 'משאית קלה',
    icon: '🚚',
    color: '#00ff88',
    dimensions: {
      length: 6.0,
      width: 2.2,
      height: 2.5,
      wheelbase: 3.5,
    },
    weight: {
      empty: 3500,
      loaded: 7500,
      axles: 2,
    },
    restrictions: {
      maxGrade: 15,
      minTurningRadius: 7.0,
      maxRoadWidth: 2.4,
      avoidTunnels: false,
      avoidBridges: false,
      hazmatAllowed: true,
      restrictions: ['low_bridges', 'narrow_roads'],
    },
    fuelType: 'diesel',
    averageSpeed: 60,
  },
  truck_medium: {
    type: 'truck_medium',
    nameEn: 'Medium Truck',
    nameHe: 'משאית בינונית',
    icon: '🚛',
    color: '#00cc66',
    dimensions: {
      length: 9.0,
      width: 2.5,
      height: 3.0,
      wheelbase: 5.0,
    },
    weight: {
      empty: 7500,
      loaded: 18000,
      axles: 3,
    },
    restrictions: {
      maxGrade: 12,
      minTurningRadius: 9.0,
      maxRoadWidth: 2.6,
      avoidTunnels: false,
      avoidBridges: true,
      hazmatAllowed: true,
      restrictions: ['low_bridges', 'narrow_roads', 'steep_hills', 'weight_restricted_roads'],
    },
    fuelType: 'diesel',
    averageSpeed: 50,
  },
  truck_heavy: {
    type: 'truck_heavy',
    nameEn: 'Heavy Truck',
    nameHe: 'משאית כבדה',
    icon: '🚛',
    color: '#00aa55',
    dimensions: {
      length: 16.5,
      width: 2.6,
      height: 3.5,
      wheelbase: 7.0,
    },
    weight: {
      empty: 15000,
      loaded: 40000,
      axles: 4,
    },
    restrictions: {
      maxGrade: 8,
      minTurningRadius: 12.0,
      maxRoadWidth: 2.8,
      avoidTunnels: true,
      avoidBridges: true,
      hazmatAllowed: true,
      restrictions: ['low_bridges', 'narrow_roads', 'steep_hills', 'weight_restricted_roads', 'residential_areas'],
    },
    fuelType: 'diesel',
    averageSpeed: 40,
  },
  bus: {
    type: 'bus',
    nameEn: 'Bus',
    nameHe: 'אוטובוס',
    icon: '🚌',
    color: '#aa66ff',
    dimensions: {
      length: 12.0,
      width: 2.5,
      height: 3.2,
      wheelbase: 6.0,
    },
    weight: {
      empty: 10000,
      loaded: 18000,
      axles: 3,
    },
    restrictions: {
      maxGrade: 10,
      minTurningRadius: 10.0,
      maxRoadWidth: 2.6,
      avoidTunnels: false,
      avoidBridges: false,
      hazmatAllowed: false,
      restrictions: ['low_bridges', 'narrow_roads', 'steep_hills'],
    },
    fuelType: 'diesel',
    averageSpeed: 50,
  },
  emergency: {
    type: 'emergency',
    nameEn: 'Emergency Vehicle',
    nameHe: 'כלי חירום',
    icon: '🚨',
    color: '#ff3355',
    dimensions: {
      length: 7.0,
      width: 2.4,
      height: 2.8,
      wheelbase: 4.0,
    },
    weight: {
      empty: 4500,
      loaded: 6500,
      axles: 2,
    },
    restrictions: {
      maxGrade: 30,
      minTurningRadius: 6.0,
      maxRoadWidth: 2.5,
      avoidTunnels: false,
      avoidBridges: false,
      hazmatAllowed: false,
      restrictions: [],
    },
    fuelType: 'diesel',
    averageSpeed: 100,
  },
  military: {
    type: 'military',
    nameEn: 'Military Vehicle',
    nameHe: 'כלי צבאי',
    icon: '🛡️',
    color: '#00e5ff',
    dimensions: {
      length: 6.5,
      width: 2.5,
      height: 2.3,
      wheelbase: 3.8,
    },
    weight: {
      empty: 5000,
      loaded: 8000,
      axles: 2,
    },
    restrictions: {
      maxGrade: 35,
      minTurningRadius: 6.5,
      maxRoadWidth: 2.6,
      avoidTunnels: false,
      avoidBridges: false,
      hazmatAllowed: false,
      restrictions: [],
    },
    fuelType: 'diesel',
    averageSpeed: 90,
  },
  van: {
    type: 'van',
    nameEn: 'Van',
    nameHe: 'פנגון',
    icon: '🚐',
    color: '#ff9900',
    dimensions: {
      length: 5.5,
      width: 2.0,
      height: 2.2,
      wheelbase: 3.2,
    },
    weight: {
      empty: 2000,
      loaded: 3500,
      axles: 2,
    },
    restrictions: {
      maxGrade: 20,
      minTurningRadius: 6.0,
      maxRoadWidth: 2.1,
      avoidTunnels: false,
      avoidBridges: false,
      hazmatAllowed: false,
      restrictions: ['low_bridges'],
    },
    fuelType: 'diesel',
    averageSpeed: 70,
  },
  bicycle: {
    type: 'bicycle',
    nameEn: 'Bicycle',
    nameHe: 'אופניים',
    icon: '🚴',
    color: '#ffd700',
    dimensions: {
      length: 1.8,
      width: 0.6,
      height: 1.0,
      wheelbase: 1.0,
    },
    weight: {
      empty: 15,
      loaded: 25,
      axles: 2,
    },
    restrictions: {
      maxGrade: 50,
      minTurningRadius: 1.5,
      maxRoadWidth: 0.7,
      avoidTunnels: true,
      avoidBridges: false,
      hazmatAllowed: false,
      restrictions: [],
    },
    fuelType: 'none',
    averageSpeed: 20,
  },
  pedestrian: {
    type: 'pedestrian',
    nameEn: 'Pedestrian',
    nameHe: 'הולך רגל',
    icon: '🚶',
    color: '#ff44aa',
    dimensions: {
      length: 0.5,
      width: 0.5,
      height: 1.7,
      wheelbase: 0.0,
    },
    weight: {
      empty: 70,
      loaded: 100,
      axles: 0,
    },
    restrictions: {
      maxGrade: 100,
      minTurningRadius: 0.5,
      maxRoadWidth: 0.5,
      avoidTunnels: true,
      avoidBridges: false,
      hazmatAllowed: false,
      restrictions: [],
    },
    fuelType: 'none',
    averageSpeed: 5,
  },
};

/**
 * Get vehicle profile by type
 */
export function getVehicleProfile(type: VehicleType): VehicleProfile {
  return VEHICLE_PROFILES[type];
}

/**
 * Get all vehicle types
 */
export function getAllVehicleTypes(): VehicleType[] {
  return Object.keys(VEHICLE_PROFILES) as VehicleType[];
}

/**
 * Check if vehicle can pass through a route segment
 */
export function canVehiclePass(
  profile: VehicleProfile,
  segmentHeight: number,
  segmentWidth: number,
  segmentGrade: number
): { canPass: boolean; reason?: string } {
  if (segmentHeight > 0 && profile.dimensions.height > segmentHeight) {
    return { canPass: false, reason: 'Vehicle too tall for this segment' };
  }

  if (segmentWidth > 0 && profile.dimensions.width > segmentWidth) {
    return { canPass: false, reason: 'Vehicle too wide for this segment' };
  }

  if (segmentGrade > profile.restrictions.maxGrade) {
    return { canPass: false, reason: 'Grade too steep for this vehicle' };
  }

  return { canPass: true };
}
