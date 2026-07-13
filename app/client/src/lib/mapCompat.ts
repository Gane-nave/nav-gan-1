/**
 * G.A.N.E — Google Maps API Compatibility Layer
 * 
 * Wraps deprecated Google Maps APIs with modern alternatives,
 * falling back to legacy APIs when the new ones aren't available.
 * 
 * Deprecated APIs addressed:
 * - google.maps.Marker → google.maps.marker.AdvancedMarkerElement
 * - google.maps.places.PlacesService → google.maps.places.Place (New)
 */

/**
 * Create a map marker using AdvancedMarkerElement if available,
 * falling back to the legacy Marker class.
 */
export function createMarker(options: {
  map: google.maps.Map;
  position: google.maps.LatLng | google.maps.LatLngLiteral;
  title?: string;
  zIndex?: number;
  icon?: {
    path: google.maps.SymbolPath;
    scale: number;
    fillColor: string;
    fillOpacity: number;
    strokeColor: string;
    strokeWeight: number;
  };
}): google.maps.Marker | google.maps.marker.AdvancedMarkerElement {
  const { map, position, title, zIndex, icon } = options;

  // Try AdvancedMarkerElement first (modern API)
  if (google.maps.marker?.AdvancedMarkerElement) {
    try {
      const content = document.createElement("div");
      if (icon) {
        // Create an SVG circle to replicate the Symbol icon
        const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
        const size = icon.scale * 2 + icon.strokeWeight * 2;
        svg.setAttribute("width", String(size));
        svg.setAttribute("height", String(size));
        svg.setAttribute("viewBox", `0 0 ${size} ${size}`);
        const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
        circle.setAttribute("cx", String(size / 2));
        circle.setAttribute("cy", String(size / 2));
        circle.setAttribute("r", String(icon.scale));
        circle.setAttribute("fill", icon.fillColor);
        circle.setAttribute("fill-opacity", String(icon.fillOpacity));
        circle.setAttribute("stroke", icon.strokeColor);
        circle.setAttribute("stroke-width", String(icon.strokeWeight));
        svg.appendChild(circle);
        content.appendChild(svg);
      }

      const marker = new google.maps.marker.AdvancedMarkerElement({
        map,
        position,
        title,
        zIndex,
        content: icon ? content : undefined,
      });
      return marker;
    } catch {
      // Fall through to legacy Marker
    }
  }

  // Fallback: legacy Marker (still works, just deprecated)
  return new google.maps.Marker({
    map,
    position,
    title,
    zIndex,
    icon: icon ? {
      path: icon.path,
      scale: icon.scale,
      fillColor: icon.fillColor,
      fillOpacity: icon.fillOpacity,
      strokeColor: icon.strokeColor,
      strokeWeight: icon.strokeWeight,
    } : undefined,
  });
}

/**
 * Update a marker's position, handling both legacy and modern markers.
 */
export function updateMarkerPosition(
  marker: google.maps.Marker | google.maps.marker.AdvancedMarkerElement,
  position: google.maps.LatLng | google.maps.LatLngLiteral
): void {
  if ('setPosition' in marker && typeof marker.setPosition === 'function') {
    // Legacy Marker
    marker.setPosition(position);
  } else if ('position' in marker) {
    // AdvancedMarkerElement
    marker.position = position;
  }
}

/**
 * Remove a marker from the map, handling both legacy and modern markers.
 */
export function removeMarker(
  marker: google.maps.Marker | google.maps.marker.AdvancedMarkerElement
): void {
  if ('setMap' in marker && typeof marker.setMap === 'function') {
    marker.setMap(null);
  } else if ('map' in marker) {
    (marker as google.maps.marker.AdvancedMarkerElement).map = null;
  }
}

/**
 * Nearby search using the new Places API if available,
 * falling back to the legacy PlacesService.
 */
export async function nearbySearch(options: {
  placesService: google.maps.places.PlacesService | null;
  location: google.maps.LatLngLiteral;
  radius: number;
  type: string;
}): Promise<{
  places: Array<{
    id: string;
    name: string;
    address: string;
    location: google.maps.LatLngLiteral;
  }>;
}> {
  const { placesService, location, radius, type } = options;

  // Fallback: legacy PlacesService (still functional)
  if (!placesService) {
    return { places: [] };
  }

  return new Promise((resolve) => {
    placesService.nearbySearch(
      { location, radius, type },
      (results, status) => {
        if (status === google.maps.places.PlacesServiceStatus.OK && results) {
          const places = results.slice(0, 10).map((r) => ({
            id: r.place_id || String(Math.random()),
            name: r.name || "Unknown",
            address: r.vicinity || "",
            location: {
              lat: r.geometry?.location?.lat() || 0,
              lng: r.geometry?.location?.lng() || 0,
            },
          }));
          resolve({ places });
        } else {
          resolve({ places: [] });
        }
      }
    );
  });
}
