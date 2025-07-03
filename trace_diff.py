#!/usr/bin/env python3
"""
Game Boy Emulator Trace Comparison Tool

Compares two execution trace files and highlights differences in CPU state.
Supports both standard format and JSON format traces.

Usage:
    python trace_diff.py trace1.txt trace2.txt
    python trace_diff.py trace1.json trace2.json
    python trace_diff.py --help
"""

import sys
import json
import re
import argparse
from dataclasses import dataclass
from typing import List, Optional, Tuple


@dataclass
class CPUState:
    """Represents the CPU state at a single instruction."""
    instruction_num: int
    a: str
    f: str
    b: str
    c: str
    d: str
    e: str
    h: str
    l: str
    sp: str
    pc: str
    memory: List[str]


class TraceParser:
    """Parses both standard and JSON format traces."""
    
    @staticmethod
    def parse_standard_line(line: str, instruction_num: int) -> Optional[CPUState]:
        """Parse a standard format trace line."""
        # Format: A: 01 F: B0 B: 00 C: 13 D: 00 E: D8 H: 01 L: 4D SP: FFFE PC: 00:0100 (00 C3 13 02)
        pattern = r'A: ([0-9A-F]{2}) F: ([0-9A-F]{2}) B: ([0-9A-F]{2}) C: ([0-9A-F]{2}) D: ([0-9A-F]{2}) E: ([0-9A-F]{2}) H: ([0-9A-F]{2}) L: ([0-9A-F]{2}) SP: ([0-9A-F]{4}) PC: 00:([0-9A-F]{4}) \(([0-9A-F]{2}) ([0-9A-F]{2}) ([0-9A-F]{2}) ([0-9A-F]{2})\)'
        
        match = re.match(pattern, line.strip())
        if not match:
            return None
            
        groups = match.groups()
        return CPUState(
            instruction_num=instruction_num,
            a=groups[0], f=groups[1], b=groups[2], c=groups[3],
            d=groups[4], e=groups[5], h=groups[6], l=groups[7],
            sp=groups[8], pc=groups[9],
            memory=[groups[10], groups[11], groups[12], groups[13]]
        )
    
    @staticmethod
    def parse_json_file(filename: str) -> List[CPUState]:
        """Parse a JSON format trace file."""
        with open(filename, 'r') as f:
            data = json.load(f)
        
        states = []
        for entry in data:
            if isinstance(entry, dict):
                states.append(CPUState(
                    instruction_num=entry["instruction"],
                    a=entry["A"], f=entry["F"], b=entry["B"], c=entry["C"],
                    d=entry["D"], e=entry["E"], h=entry["H"], l=entry["L"],
                    sp=entry["SP"], pc=entry["PC"],
                    memory=entry["memory"]
                ))
        return states
    
    @staticmethod
    def parse_standard_file(filename: str) -> List[CPUState]:
        """Parse a standard format trace file."""
        states = []
        with open(filename, 'r') as f:
            for i, line in enumerate(f):
                state = TraceParser.parse_standard_line(line, i)
                if state:
                    states.append(state)
        return states
    
    @staticmethod
    def parse_file(filename: str) -> List[CPUState]:
        """Auto-detect format and parse trace file."""
        try:
            # Try JSON first
            return TraceParser.parse_json_file(filename)
        except (json.JSONDecodeError, KeyError):
            # Fall back to standard format
            return TraceParser.parse_standard_file(filename)


class TraceDiffer:
    """Compares two trace files and highlights differences."""
    
    def __init__(self, colorize: bool = True):
        self.colorize = colorize
        
        # ANSI color codes
        self.colors = {
            'red': '\033[31m',
            'green': '\033[32m',
            'yellow': '\033[33m',
            'blue': '\033[34m',
            'magenta': '\033[35m',
            'cyan': '\033[36m',
            'reset': '\033[0m',
            'bold': '\033[1m'
        } if colorize else {k: '' for k in ['red', 'green', 'yellow', 'blue', 'magenta', 'cyan', 'reset', 'bold']}
    
    def colorize_text(self, text: str, color: str) -> str:
        """Apply color to text if colorization is enabled."""
        return f"{self.colors[color]}{text}{self.colors['reset']}"
    
    def compare_states(self, state1: CPUState, state2: CPUState) -> Tuple[bool, List[str]]:
        """Compare two CPU states and return differences."""
        differences = []
        has_differences = False
        
        # Compare registers
        registers = [
            ('A', state1.a, state2.a),
            ('F', state1.f, state2.f),
            ('B', state1.b, state2.b),
            ('C', state1.c, state2.c),
            ('D', state1.d, state2.d),
            ('E', state1.e, state2.e),
            ('H', state1.h, state2.h),
            ('L', state1.l, state2.l),
            ('SP', state1.sp, state2.sp),
            ('PC', state1.pc, state2.pc)
        ]
        
        for reg_name, val1, val2 in registers:
            if val1 != val2:
                has_differences = True
                diff_text = f"{reg_name}: {self.colorize_text(val1, 'red')} -> {self.colorize_text(val2, 'green')}"
                differences.append(diff_text)
        
        # Compare memory
        if state1.memory != state2.memory:
            has_differences = True
            mem1_str = ' '.join(state1.memory)
            mem2_str = ' '.join(state2.memory)
            diff_text = f"MEM: ({self.colorize_text(mem1_str, 'red')}) -> ({self.colorize_text(mem2_str, 'green')})"
            differences.append(diff_text)
        
        return has_differences, differences
    
    def compare_traces(self, trace1: List[CPUState], trace2: List[CPUState], 
                      max_lines: Optional[int] = None, show_matching: bool = False,
                      limit_comparison: Optional[int] = None) -> None:
        """Compare two traces and output differences."""
        min_len = min(len(trace1), len(trace2))
        max_len = max(len(trace1), len(trace2))
        
        # Apply comparison limit if specified
        if limit_comparison is not None:
            min_len = min(min_len, limit_comparison)
            original_max_len = max_len
            max_len = min(max_len, limit_comparison)
            
            if limit_comparison < original_max_len:
                print(f"{self.colorize_text('INFO:', 'cyan')} Limiting comparison to first {limit_comparison} instructions")
                print()
        
        if len(trace1) != len(trace2):
            print(f"{self.colorize_text('WARNING:', 'yellow')} Trace lengths differ: {len(trace1)} vs {len(trace2)}")
            print()
        
        differences_found = 0
        lines_processed = 0
        
        for i in range(min_len):
            if max_lines and lines_processed >= max_lines:
                break
                
            state1, state2 = trace1[i], trace2[i]
            has_diff, diffs = self.compare_states(state1, state2)
            
            if has_diff or show_matching:
                lines_processed += 1
                
                if has_diff:
                    differences_found += 1
                    print(f"{self.colorize_text(f'DIFF at instruction {i}:', 'bold')}")
                    for diff in diffs:
                        print(f"  {diff}")
                    print()
                elif show_matching:
                    print(f"{self.colorize_text(f'MATCH at instruction {i}:', 'blue')} PC: {state1.pc}")
        
        # Handle extra lines in longer trace
        if min_len < max_len:
            longer_trace = trace1 if len(trace1) > len(trace2) else trace2
            trace_name = "trace1" if len(trace1) > len(trace2) else "trace2"
            
            print(f"{self.colorize_text(f'Extra lines in {trace_name}:', 'yellow')}")
            for i in range(min_len, min(max_len, min_len + 10)):  # Show up to 10 extra lines
                state = longer_trace[i]
                print(f"  Instruction {i}: PC: {state.pc}")
            
            if max_len > min_len + 10:
                print(f"  ... and {max_len - min_len - 10} more lines")
            print()
        
        # Summary
        total_compared = min(lines_processed, min_len)
        print(f"{self.colorize_text('SUMMARY:', 'bold')}")
        print(f"  Instructions compared: {total_compared}")
        print(f"  Differences found: {self.colorize_text(str(differences_found), 'red' if differences_found > 0 else 'green')}")
        
        if differences_found == 0 and len(trace1) == len(trace2):
            print(f"  {self.colorize_text('✓ Traces are identical!', 'green')}")
        elif differences_found == 0:
            print(f"  {self.colorize_text('✓ No differences in compared instructions', 'green')}")


def main():
    parser = argparse.ArgumentParser(
        description="Compare Game Boy emulator execution traces",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python trace_diff.py trace1.txt trace2.txt
  python trace_diff.py trace1.json trace2.json --no-color
  python trace_diff.py trace1.txt trace2.txt --max-lines 50 --show-matching
  python trace_diff.py trace1.txt trace2.txt --limit-comparison 1000
        """
    )
    
    parser.add_argument('trace1', help='First trace file')
    parser.add_argument('trace2', help='Second trace file')
    parser.add_argument('--no-color', action='store_true', help='Disable colored output')
    parser.add_argument('--max-lines', type=int, help='Maximum number of differences to show')
    parser.add_argument('--show-matching', action='store_true', help='Show matching instructions too')
    parser.add_argument('--limit-comparison', type=int, help='Only compare the first N instructions')
    
    args = parser.parse_args()
    
    try:
        print(f"Loading {args.trace1}...")
        trace1 = TraceParser.parse_file(args.trace1)
        
        print(f"Loading {args.trace2}...")
        trace2 = TraceParser.parse_file(args.trace2)
        
        print(f"Loaded {len(trace1)} and {len(trace2)} instructions respectively.")
        print()
        
        differ = TraceDiffer(colorize=not args.no_color)
        differ.compare_traces(trace1, trace2, args.max_lines, args.show_matching, args.limit_comparison)
        
    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"Error parsing JSON: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()