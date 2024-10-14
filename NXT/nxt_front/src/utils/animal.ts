export class animalutil {
  static stp: number = 1
  static max_wdth: number
  static min_wdth: number
  public static dft_wdth: number

  constructor(dftWdth: number, maxWdth: number, minWdth: number) {
    animalutil.dft_wdth = dftWdth
    animalutil.max_wdth = maxWdth
    animalutil.min_wdth = minWdth
  }

  static wdth_expand() {
    // console.log(animalutil.dft_wdth, animalutil.max_wdth)
    if (animalutil.dft_wdth < animalutil.max_wdth) {
      animalutil.dft_wdth += animalutil.stp
      // animalutil.ele.style.width = `${animalutil.dft_wdth}px`
      requestAnimationFrame(animalutil.wdth_expand)
    }
  }

  static wdth_shrink() {
    // console.log(animalutil.dft_wdth, animalutil.min_wdth)
    if (animalutil.dft_wdth > animalutil.min_wdth) {
      animalutil.dft_wdth -= animalutil.stp
      // animalutil.ele.style.width = `${animalutil.dft_wdth}px`
      requestAnimationFrame(animalutil.wdth_shrink)
    }
  }
}
