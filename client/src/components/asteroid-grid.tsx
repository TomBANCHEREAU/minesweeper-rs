import React, { useCallback } from "preact/compat";
import "./asteroid-grid.css";

export function AsteroidGrid({
  content,
  onClick,
  onDblClick,
  onContextMenu,
}: {
  content: unknown[][];
  onClick: (x: number, y: number, cell: unknown) => void;
  onDblClick: (x: number, y: number, cell: unknown) => void;
  onContextMenu: (x: number, y: number, cell: unknown) => void;
}) {
  return (
    <div class="asteroid-grid">
      {content.map((row, y) => (
        <div key={`row-${y}`}>
          {row.map((cell, x) => (
            <Cell
              key={`cell-${x}-${y}`}
              x={x}
              y={y}
              content={cell}
              onClick={onClick}
              onDblClick={onDblClick}
              onContextMenu={onContextMenu}
            ></Cell>
          ))}
        </div>
      ))}
    </div>
  );
}

const Cell = React.memo<{
  content: unknown;
  x: number;
  y: number;
  onClick: (x: number, y: number, cell: unknown) => void;
  onDblClick: (x: number, y: number, cell: unknown) => void;
  onContextMenu: (x: number, y: number, cell: unknown) => void;
}>(({ content, x, y, ...callbacks }) => {
  const onClick = useCallback(
    () => callbacks.onClick(x, y, content),
    [callbacks.onClick, x, y, content]
  );
  const onDblClick = useCallback(
    () => callbacks.onDblClick(x, y, content),
    [callbacks.onDblClick, x, y, content]
  );
  const onContextMenu = useCallback(
    (event: Event) => {
      event.preventDefault();
      return callbacks.onContextMenu(x, y, content);
    },
    [callbacks.onContextMenu, x, y, content]
  );

  return (
    <button
      onClick={onClick}
      onDblClick={onDblClick}
      onContextMenu={onContextMenu}
      class={content as string}
    ></button>
  );
});

// const Cell = React.memo<{
//   cell: MinesweeperCell;
//   setFlag: () => void;
//   removeFlag: () => void;
//   discover: () => void;
//   discoverAround: () => void;
// }>(({ cell, discover, removeFlag, setFlag, discoverAround }) => {
//   if (cell.bomb)
//     return (
//       <Box
//         component="button"
//         sx={[
//           ...(cell.discovered
//             ? []
//             : [
//                 ({ palette }: Theme) => ({
//                   backgroundColor: palette.primary.main,
//                   borderWidth: "4px",
//                 }),
//               ]),
//           {
//             fontSize: "1.2rem",
//             fontWeight: "800",
//             color: "red",
//             p: 0,
//           },
//         ]}
//       >
//         <svg
//           version="1.0"
//           xmlns="http://www.w3.org/2000/svg"
//           viewBox="0 0 24 24"
//         >
//           <circle cx="12" cy="12" r="6" />
//           <polygon points="3,12 9,9 12,3 15,9 21,12 15,15 12,21 9,15" />
//           <polygon
//             transform="rotate(45 12 12)"
//             points="3,12 10,10 12,3 14,10 21,12 14,14 12,21 10,14"
//           />
//         </svg>
//       </Box>
//     );
//   if (cell.flagged)
//     return (
//       <Box
//         component="button"
//         sx={({ palette }) => ({
//           backgroundColor: palette.primary.main,
//           color: palette.success.main,
//           borderWidth: "4px",
//           p: 0,
//         })}
//         onContextMenu={removeFlag}
//       >
//         <FlagOutlined fontSize="small"></FlagOutlined>
//       </Box>
//     );
//   if (!cell.discovered)
//     return (
//       <Box
//         component="button"
//         sx={({ palette }) => ({
//           backgroundColor: palette.primary.main,
//           borderWidth: "4px",
//         })}
//         onClick={discover}
//         onContextMenu={setFlag}
//       ></Box>
//     );
//   return (
//     <Box
//       component="button"
//       disabled={!cell.hint}
//       onDoubleClick={discoverAround}
//       sx={({ palette }) => ({
//         fontSize: "1.2rem",
//         fontWeight: "800",
//         color:
//           {
//             1: palette.primary.main,
//             2: "green",
//             3: "red",
//             4: "blue",
//             5: "darkred",
//             6: "darkcyan",
//             7: "black",
//           }[cell.hint ?? 0] ?? undefined,
//       })}
//     >
//       {cell.hint || ""}
//     </Box>
//   );
// });
