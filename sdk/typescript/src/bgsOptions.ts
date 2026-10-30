export type BgsConfigValue = string | number | boolean | BgsConfigValue[] | BgsConfigObject;

export type BgsConfigObject = { [key: string]: BgsConfigValue };

export type BgsOptions = {
  bgsPathOverride?: string;
  baseUrl?: string;
  apiKey?: string;
  /**
   * Additional `--config key=value` overrides to pass to the BGS CLI.
   *
   * Provide a JSON object and the SDK will flatten it into dotted paths and
   * serialize values as TOML literals so they are compatible with the CLI's
   * `--config` parsing.
   */
  config?: BgsConfigObject;
  /**
   * Raw `--config key=value` overrides to pass unchanged to the BGS CLI after
   * structured configuration and before SDK-managed or thread-specific overrides.
   */
  configOverrides?: string[];
  /**
   * Environment variables passed to the BGS CLI process. When provided, the SDK
   * will not inherit variables from `process.env`.
   */
  env?: Record<string, string>;
};
