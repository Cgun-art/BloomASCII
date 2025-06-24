package {
    import flash.display.Sprite;

    public class KeyGenerator extends Sprite {
        public function KeyGenerator() {
            var totalKeys:int = 87;
            var keyLengths:Array = [4, 7, 12, 35];
            var allKeys:Array = [];

            while (allKeys.length < totalKeys) {
                var length:int = keyLengths[Math.floor(Math.random() * keyLengths.length)];
                var key:String = generateKey(length);
                allKeys.push(key);
            }

            for (var i:int = 0; i < allKeys.length; i++) {
                trace("Key " + (i + 1) + ": " + allKeys[i]);
            }
        }

        private function generateKey(len:int):String {
            var chars:String = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            var result:String = "";

            for (var i:int = 0; i < len; i++) {
                var index:int = Math.floor(Math.random() * chars.length);
                result += chars.charAt(index);
            }

            return result;
        }
    }
}
