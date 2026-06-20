import { describe, expect, it } from "vitest";
import packageJson from "../../package.json";
import tauriConfig from "../../src-tauri/tauri.conf.json";

describe("Windows packaging config", () => {
  it("exposes one command for Windows installer packaging", () => {
    expect(packageJson.scripts["package:windows"]).toBe("tauri build --ci --bundles nsis");
  });

  it("targets a Windows NSIS installer bundle", () => {
    expect(tauriConfig.bundle.active).toBe(true);
    expect(tauriConfig.bundle.targets).toContain("nsis");
  });
});
