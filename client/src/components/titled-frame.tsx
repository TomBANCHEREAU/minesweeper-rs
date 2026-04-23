import { ComponentChildren } from "preact";
import React, { PropsWithChildren } from "preact/compat";
import "./titled-frame.css";

export const TitledFrame = React.memo<
  PropsWithChildren<{
    title: string;
    rightSlot?: ComponentChildren;
    style?: string;
    class?: string;
  }>
>(({ title, children, rightSlot, style, class: className }) => {
  return (
    <div class={"titled-frame " + (className ?? "")} style={style}>
      <div>
        <h1>{title}</h1>
        {rightSlot}
      </div>
      {children}
    </div>
  );
});
