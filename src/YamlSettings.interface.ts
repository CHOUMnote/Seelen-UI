/* eslint-disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type ApplicationIdentifier = "Exe" | "Class" | "Title";
export type MatchingStrategy = "Legacy" | "Equals" | "StartsWith" | "EndsWith" | "Contains" | "Regex";
export type ApplicationOptions =
  | "object_name_change"
  | "layered"
  | "border_overflow"
  | "tray_and_multi_window"
  | "force"
  | "unmanage"
  | "float";
export type ArrayOf_ApplicationConfiguration = ApplicationConfiguration[];

export interface ApplicationConfiguration {
  float_identifiers?: IdWithIdentifierAndComment[] | null;
  identifier: IdWithIdentifier;
  name: string;
  category?: null | string;
  binded_workspace?: null | string;
  binded_monitor?: null | number;
  options?: ApplicationOptions[] | null;
  [k: string]: unknown;
}
export interface IdWithIdentifierAndComment {
  comment?: string | null;
  id: string;
  kind: ApplicationIdentifier;
  matching_strategy?: MatchingStrategy | null;
  [k: string]: unknown;
}
export interface IdWithIdentifier {
  id: string;
  kind: ApplicationIdentifier;
  matching_strategy?: MatchingStrategy | null;
  [k: string]: unknown;
}
