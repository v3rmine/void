import {
  PropsWithChildren,
  createContext,
  useContext,
  useEffect,
  useState,
} from "react";
import { ripeService } from "./ripe/ripeService";

const { getApproximateLocation } = ripeService();

interface LocationService {
  geolocationError: GeolocationPositionError | null;
  precision: "city" | "country" | "precise" | null;
  location: {
    latitude: number;
    longitude: number;
  } | null;
}

const LocationContext = createContext<LocationService | null>(null);

const useProvideLocation = () => {
  const [precision, setPrecision] =
    useState<LocationService["precision"]>(null);
  const [location, setLocation] = useState<LocationService["location"]>(null);
  const [geolocationError, setError] =
    useState<LocationService["geolocationError"]>(null);

  // Firstly fetch approximate location
  useEffect(() => {
    void getApproximateLocation().then((location) => {
      setPrecision(location?.precision ?? null);
      setLocation(
        location
          ? {
              latitude: location.latitude,
              longitude: location.longitude,
            }
          : null,
      );
    });
  }, []);

  // Randomly move in circle
  // useEffect(() => {
  //   const interval = setInterval(() => {
  //     const { latitude, longitude } = location ?? { latitude: 0, longitude: 0 };
  //     const angle = Math.random() * Math.PI * 2;
  //     const radius = Math.random() * 0.0001;
  //     setLocation({
  //       latitude: latitude + Math.sin(angle) * radius,
  //       longitude: longitude + Math.cos(angle) * radius,
  //     });
  //   }, 200);

  //   return () => {
  //     clearInterval(interval);
  //   };
  // }, [location]);

  // Then try to watch location
  useEffect(() => {
    const watch = navigator.geolocation.watchPosition(
      (position) => {
        setLocation({
          latitude: position.coords.latitude,
          longitude: position.coords.longitude,
        });
        setError(null);
      },
      (error) => {
        setError(error);
      },
      {
        enableHighAccuracy: true,
      },
    );

    return () => {
      navigator.geolocation.clearWatch(watch);
    };
  }, []);

  return {
    location,
    precision,
    geolocationError,
  };
};

export function ProvideLocation({ children }: PropsWithChildren) {
  const locationProvider = useProvideLocation();

  return (
    <LocationContext.Provider value={locationProvider}>
      {children}
    </LocationContext.Provider>
  );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useGeolocation() {
  return useContext(LocationContext)!;
}

