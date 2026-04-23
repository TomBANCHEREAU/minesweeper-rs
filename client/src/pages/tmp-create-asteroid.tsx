import { TargetedSubmitEvent } from "preact";
import { useLocation } from "preact-iso";
import React, { useCallback } from "preact/compat";

export const AsteroidCreationForm = React.memo(() => {
  const { route } = useLocation();
  const onSubmit = useCallback(
    async (event: TargetedSubmitEvent<HTMLFormElement>) => {
      event.preventDefault();
      const elements = event.currentTarget.elements;
      const response: { id: string } = await fetch("/api/lobby", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          grid_width: parseInt(elements["grid_width"].value),
          grid_height: parseInt(elements["grid_height"].value),
        }),
      }).then((r) => r.json());
      route(`/asteroid/${response.id}`);
    },
    [],
  );
  return (
    <>
      <a href="/map">Carte</a>
      <form action="/api/lobby" method="POST" onSubmit={onSubmit}>
        <input type="number" name="grid_width" value="10" />
        <input type="number" name="grid_height" value="10" />
        <button type="submit">Valider</button>
      </form>
    </>
  );
});
