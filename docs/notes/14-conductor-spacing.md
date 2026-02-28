# Minimum Conductor Spacing (IPC-2221C)

## Overview
Displays minimum conductor spacing based on voltage and device type,
per the IPC-2221C specification.

## Inputs
- **Voltage Between Conductors** - predefined ranges:
  - 0-15V, 16-30V, 31-50V, 51-100V, 101-150V, 151-170V,
  - 171-250V, 251-300V, 301-500V, >500V (custom entry)
- **Device Type Selection** (IPC-2221C categories):
  - B1 = Internal Conductors
  - B2 = External Conductors, Uncoated, Sea Level to 3050m
  - B3 = External Conductors, Uncoated, Over 3050m or in a Vacuum
  - B4 = External Conductors Covered With Solder Mask (Any Elevation)
  - B5 = External Conductors, Coated (Any Elevation or in a Vacuum)
  - A6 = External Component Lead, Coated (Any Elevation or in a Vacuum)
  - A7 = External Component Lead, Uncoated, Sea Level to 3050m
  - A8 = External Component Lead, Uncoated, Over 3050m or in a Vacuum

## Output
- **Minimum Conductor Spacing** (mils/mm) - per IPC-2221C table

## IPC-2221C Spacing Table (approximate values)
This is a lookup table, not a formula. The values are predefined per the standard.
Example for B1 (Internal Conductors):
| Voltage | Min Spacing |
|---------|------------|
| 0-15V   | 1.97 mils  |
| 16-30V  | 2.0 mils   |
| 31-50V  | 4.0 mils   |
| etc.    | ...        |

## Implementation Notes
- Pure lookup table implementation
- For >500V, the user enters a custom voltage and spacing is interpolated/calculated
- Different tables for each device type (B1-B5, A6-A8)
- The table data needs to be extracted from the binary or sourced from the IPC-2221C standard
