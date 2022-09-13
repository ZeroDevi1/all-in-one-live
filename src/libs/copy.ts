export const copyText = (value: string) => {
    const handleCopy = (e: ClipboardEvent) => {
        // clipboardData 可能是 null
        e.clipboardData && e.clipboardData.setData('text/plain', value);
        e.preventDefault();
        // removeEventListener 要传入第二个参数
        // @ts-ignore
        document.removeEventListener('copy', handleCopy);
    };
    // 复制 VideoUrl 的数据到剪切板
    // @ts-ignore
    document.addEventListener('copy', handleCopy);
    document.execCommand('copy');

}