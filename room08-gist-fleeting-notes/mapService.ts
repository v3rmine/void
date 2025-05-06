import {
  useState,
  useRef,
  useCallback,
  createContext,
  PropsWithChildren,
  useContext,
  RefObject,
  ReactNode,
  ComponentProps,
  useMemo,
  useEffect,
} from "react";
import MapLibreMap, { MapRef, ViewState } from "react-map-gl/maplibre";
import maplibregl from "maplibre-gl";
import "maplibre-gl/dist/maplibre-gl.css";
import { loggerBuilder } from "../logger";
import { useGeolocation } from "../locationService";
import { spliceReturn } from "../../react-helpers/array";
import { randomUUID } from "../../react-helpers/crypto";

const logger = loggerBuilder("map");

const FRANCE_CENTER = {
  latitude: 46.603354,
  longitude: 1.888334,
};
// eslint-disable-next-line react-refresh/only-export-components
export const ZOOM_LEVELS = {
  city: 13,
  country: 6,
  precise: 15,
};

export const MapComponent = ({
  defaultPitch = 25,
  noControl = false,
  noAttribution = false,
}: {
  defaultPitch?: number;
  noControl?: boolean;
  noAttribution?: boolean;
}) => {
  const { mapRef, modules, modulesExtraProps } = useMap();
  const { location } = useGeolocation();

  const [initialViewState] = useState<Partial<ViewState>>({
    pitch: defaultPitch,
    bearing: 0,
    zoom: ZOOM_LEVELS.country,
    ...(location ?? FRANCE_CENTER),
  });

  const mapTilerStyleUrl =
    "https://<tileserver-gl>/styles/dataviz/style.json";

  const onMapLoad = useCallback(() => {
    const map = mapRef.current?.getMap();
    if (!map) {
      logger.error("MapLibre map instance not found.");
      return;
    }

    if (!noControl)
      map.addControl(
        new maplibregl.NavigationControl({
          showCompass: true,
          showZoom: true,
        }),
      );

    logger.debug("Map loaded. Applying style modifications...");

    // List of layer IDs to hide. Inspect the JSON style to find the right IDs.
    // Common examples: points of interest (poi), buildings (building), certain labels.
    const layersToRemoveOrHide: string[] = [];

    layersToRemoveOrHide.forEach((layerId) => {
      if (map.getLayer(layerId)) {
        try {
          // Using setLayoutProperty to hide is safer than removeLayer
          map.setLayoutProperty(layerId, "visibility", "none");
          logger.debug(`Layer hidden: ${layerId}`);
        } catch (error) {
          logger.info(`Unable to hide layer ${layerId}: `, error);
        }
      } else {
        logger.info(
          `Layer to hide not found: ${layerId}. IDs may vary depending on the style.`,
        );
      }
    });

    logger.debug("Style modifications completed.");
  }, [mapRef, noControl]);

  // When precision changes, update the map center and zoom level

  return (
    <div style={{ width: "100%", position: "relative", alignSelf: "stretch" }}>
      <MapLibreMap
        ref={mapRef}
        mapLib={maplibregl}
        initialViewState={initialViewState}
        style={{ width: "100%", height: "100%" }}
        mapStyle={mapTilerStyleUrl}
        onLoad={onMapLoad}
        interactive={!noControl}
        attributionControl={noAttribution ? false : undefined}
      >
        {modules.map(({ id, component: Module }) => (
          <Module key={id} mapRef={mapRef} {...modulesExtraProps[id]} />
        ))}
      </MapLibreMap>
    </div>
  );
};

type ModuleType<ExtraProps extends Record<string, unknown>> = (
  props:
    | {
        mapRef: MapService["mapRef"];
      }
    | ({
        mapRef: MapService["mapRef"];
      } & ExtraProps)
    | ExtraProps,
) => ReactNode;

interface ModuleTypeObject<ExtraProps extends Record<string, unknown>> {
  id: string;
  component: ModuleType<ExtraProps>;
}

interface MapService {
  mapRef: RefObject<MapRef>;

  modules: ModuleTypeObject<any>[];
  modulesExtraProps: Record<string, any>;

  useRegisterModule: <C extends ModuleType<any>>(
    component: C,
    extraProps?: Omit<ComponentProps<C>, "mapRef">,
  ) => void;
}

const MapContext = createContext<MapService | null>(null);

const useProvideLocation = (): MapService => {
  const mapRef = useRef<MapRef>(null);

  const [modules, setModules] = useState<ModuleTypeObject<any>[]>([]);
  const [modulesExtraProps, setModulesExtraProps] = useState<
    Record<string, any>
  >({});

  logger.debug("loaded modules: ", modules);

  const useRegisterModule: MapService["useRegisterModule"] =
    function useRegisterModule(component, extraProps) {
      const module = useMemo(
        () => ({
          id: randomUUID(),
          component,
        }),
        [component],
      );
      const memoizedExtraProps = useMemo(
        () => ({ ...extraProps }),
        // eslint-disable-next-line react-hooks/exhaustive-deps
        extraProps ? Object.values(extraProps) : [],
      );

      // If component change we reload the module
      useEffect(() => {
        setModules((prev) => [...prev, module]);

        return () => {
          setModules((prev) => spliceReturn(prev, prev.indexOf(module)));
        };
      }, [module]);

      // If extraProps change we update the module's extraProps
      useEffect(() => {
        setModulesExtraProps((prev) => ({
          ...prev,
          [module.id]: memoizedExtraProps,
        }));

        return () => {
          setModulesExtraProps((prev) => {
            const newProps = { ...prev };
            delete newProps[module.id];
            return newProps;
          });
        };
      }, [module.id, memoizedExtraProps]);
    };

  return {
    mapRef,
    useRegisterModule,
    modules,
    modulesExtraProps,
  };
};

export function ProvideMap({ children }: PropsWithChildren) {
  const locationProvider = useProvideLocation();

  return (
    <MapContext.Provider value={locationProvider}>
      {children}
    </MapContext.Provider>
  );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useMap() {
  return useContext(MapContext)!;
}
