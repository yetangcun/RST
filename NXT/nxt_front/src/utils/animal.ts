export class animalutil {
  static stp: number = 1
  static max_wdth: number
  static min_wdth: number
  static ele: HTMLElement | null
  public static dft_wdth: number

  constructor(eleId: string, maxWdth: number, minWdth: number) {
    animalutil.dft_wdth = maxWdth
    animalutil.max_wdth = maxWdth
    animalutil.min_wdth = minWdth
    animalutil.ele = document.getElementById(eleId)
  }

  static wdth_expand() {
    if (animalutil.dft_wdth < animalutil.max_wdth) {
      animalutil.dft_wdth += animalutil.stp * 5
      if (animalutil.ele) {
        animalutil.ele.style.maxWidth = `${animalutil.dft_wdth}px`
        animalutil.ele.style.minWidth = `${animalutil.dft_wdth}px`
      }
      requestAnimationFrame(animalutil.wdth_expand)
    }
  }

  static wdth_shrink() {
    if (animalutil.dft_wdth > animalutil.min_wdth) {
      animalutil.dft_wdth -= animalutil.stp * 5
      if (animalutil.ele) {
        animalutil.ele.style.minWidth = `${animalutil.dft_wdth}px`
        animalutil.ele.style.maxWidth = `${animalutil.dft_wdth}px`
      }
      requestAnimationFrame(animalutil.wdth_shrink)
    }
  }
}
