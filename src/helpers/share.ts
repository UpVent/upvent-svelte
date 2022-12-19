export async function share(): Promise<void> {
    if (navigator.canShare) {
        navigator.share({ 
            title: 'UpVent Technologies',
            text: '¡Visita UpVent!',
            url: 'https://upvent.codes/'
        }); 
    } else {
        return;
    }
}