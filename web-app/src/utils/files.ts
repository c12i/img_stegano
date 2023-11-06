export const getAsByteArray = async (file: any) =>  {
  return new Uint8Array(await readFile(file) as any)
}

const readFile = (file: any) => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
		// @ts-ignore
    reader.addEventListener("loadend", e => resolve(e.target.result))
    reader.addEventListener("error", reject)
    // Read file
    reader.readAsArrayBuffer(file)
  })
}