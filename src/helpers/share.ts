export async function share(shareText: string): Promise<void> {
    if (navigator.canShare) {
        navigator.share({ 
            title: 'UpVent Technologies',
            text: shareText,
            url: 'https://upvent.codes/'
        }); 
    } else {
        return;
    }
}