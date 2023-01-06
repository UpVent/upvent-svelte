/**
 * Opens a "Share" dialog on specific pages on sharing-capable
 * browsers, mostly mobile browsers.
 *
 * @export
 * @param {string} shareText
 * @return {*}  {Promise<void>}
 */
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
