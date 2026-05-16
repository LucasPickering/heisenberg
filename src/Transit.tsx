import React from "react";
import { TransitPredictions } from "./state.ts";

function Transit(
  { transit }: { transit: TransitPredictions },
): React.ReactNode {
  return (
    <table>
      <tbody>
        {transit.lines.map((line) => (
          <React.Fragment key={line.name}>
            <tr>
              <th>{line.name}</th>
            </tr>

            {line.stops.map(stop => (
              <tr key={stop.name}>
                <td>{stop.name}</td>
                <td>
                  {stop.predictions.map(minutes => `${minutes}m`).join(", ")}
                </td>
              </tr>
            ))}
          </React.Fragment>
        ))}
      </tbody>
    </table>
  );
}

export default Transit;
