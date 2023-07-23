import React from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Process() {
  let processes;
  let func = () =>
    invoke("processes").then((res) => {
      processes = res;
      console.log(processes);
    });
  return (
    <button
      style={{ color: "red", margin: "100px" }}
      onClick={() => {
        func();
      }}
    ></button>
  );
}
