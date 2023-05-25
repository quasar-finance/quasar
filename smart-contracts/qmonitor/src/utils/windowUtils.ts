import { SIDEBAR_WIDTH } from './config'

export function clamp (min: number, max: number, target: number) {
  return Math.max(min, Math.min(max, target))
}

export function getCenterColWidth (width: number) {
  return Math.floor(width * 0.3) > SIDEBAR_WIDTH
    ? width - SIDEBAR_WIDTH * 2
    : Math.floor(width * 0.4)
}
