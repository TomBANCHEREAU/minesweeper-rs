import { useRoute } from "preact-iso";
import { FC, useCallback, useState } from "preact/compat";
import { AsteroidGrid } from "../components/asteroid-grid";
import { TitledFrame } from "../components/titled-frame";
import { useEvent } from "../hooks/use-event";
import { useSocket } from "../hooks/use-socket";

export const useAsteroid = (asteroidId: string) => {
  const { socket, status } = useSocket(`/api/lobby/${asteroidId}/ws`);
  const [grid, setGrid] = useState<string[][]>();
  useEvent(
    socket,
    "message",
    (event: MessageEvent<string>) => {
      const { GameEvent: { GameStart, TileStateUpdate } = {} } = JSON.parse(
        event.data
      ) as {
        GameEvent?: {
          GameStart?: {
            grid: {
              width: number;
              grid: (string | { Discovered: string | { Number: string } })[];
            };
            cursors: {};
          };
          TileStateUpdate: {
            x: number;
            y: number;
            state: { Discovered: string | { Number: string } };
          };
        };
      };
      if (GameStart) {
        const grid = [];
        while (GameStart.grid.grid.length) {
          grid.push(
            GameStart.grid.grid.splice(0, GameStart.grid.width).map((state) => {
              if (typeof state === "string") return state;
              const { Discovered } = state;
              return typeof Discovered === "string"
                ? Discovered
                : Discovered.Number;
            })
          );
        }
        setGrid(grid);
      }
      if (TileStateUpdate) {
        setGrid((g) => {
          if (typeof TileStateUpdate.state === "string")
            g[TileStateUpdate.y][TileStateUpdate.x] = TileStateUpdate.state;
          else {
            const { Discovered } = TileStateUpdate.state;
            g[TileStateUpdate.y][TileStateUpdate.x] =
              typeof Discovered === "string" ? Discovered : Discovered.Number;
          }
          return [...g];
        });
      }
    },
    []
  );
  const discoverTile = useCallback(
    (x: number, y: number) => {
      socket.send(JSON.stringify({ GameAction: { Discover: { x, y } } }));
    },
    [socket]
  );
  const toggleFlag = useCallback(
    (x: number, y: number, state: string) => {
      socket.send(
        JSON.stringify({
          GameAction:
            state === "Flagged"
              ? { RemoveFlag: { x, y } }
              : { PlaceFlag: { x, y } },
        })
      );
    },
    [socket]
  );
  return {
    grid,
    status: grid ? status : "connecting",
    discoverTile,
    toggleFlag,
  };
};

export const AsteroidPage: FC = () => {
  const {
    params: { asteroidId },
  } = useRoute();
  const { grid, status, discoverTile, toggleFlag } = useAsteroid(asteroidId);

  return (
    <TitledFrame
      title="Astéroïde"
      rightSlot={status}
      style="width: 100vw;height: calc(100vh - 2px)"
    >
      {status === "open" && (
        <AsteroidGrid
          content={grid}
          onClick={discoverTile}
          onContextMenu={toggleFlag}
        ></AsteroidGrid>
      )}
    </TitledFrame>
  );
};
