import { describe, expect, it } from "vitest";
import fs from "node:fs";
import path from "node:path";
import { AuditEngine } from "../src/engine";
import { createDefaultRegistry } from "../src/registry";

const CONTRACT = path.join(
  process.cwd(),
  "contracts",
  "StorageTtlEdgeCases.rs",
);

describe("Storage TTL edge cases (AP-STORAGE-001)", () => {
  it("accepts persistent and temporary TTL extensions across conditional branches", () => {
    const source = fs.readFileSync(CONTRACT, "utf-8");
    const findings = new AuditEngine(createDefaultRegistry()).run(source);

    expect(findings.filter((finding) => finding.id === "AP-STORAGE-001")).toEqual(
      [],
    );
  });

  it("reports the storage access when both tier-specific TTL extensions are absent", () => {
    const source = fs.readFileSync(CONTRACT, "utf-8").replace(
      /\.extend_ttl\(&key, \d+, \d+\);/g,
      "",
    );
    const findings = new AuditEngine(createDefaultRegistry()).run(source);
    const storageFindings = findings.filter(
      (finding) => finding.id === "AP-STORAGE-001",
    );

    expect(storageFindings).toHaveLength(1);
    expect(storageFindings[0]?.location.function).toBe("refresh");
  });
});
