// The initial version of this file was autogenerated from the official AS3 reference at 
// https://help.adobe.com/en_US/FlashPlatform/reference/actionscript/3/flash/events/SampleDataEvent.html
// by https://github.com/golfinq/ActionScript_Event_Builder
// It won't be regenerated in the future, so feel free to edit and/or fix
package flash.events
{
    
    import flash.utils.ByteArray;
    
    public class SampleDataEvent extends Event
    {
        public static const SAMPLE_DATA:String = "sampleData"; // Defines the value of the type property of a SampleDataEvent event object.

        private var _position: Number; // The position of the data in the audio stream.
        private var _data: ByteArray; // The data in the audio stream.

        public function SampleDataEvent(type:String, bubbles:Boolean = false, cancelable:Boolean = false, theposition:Number = 0, thedata:ByteArray = null)
        {
            super(type,bubbles,cancelable);
            this.position = theposition;
            this.data = thedata;
        }

        public function get position():Number {
            return this._position;
        }
        public function set position(value:Number):void {
            this._position = value;
        }

        public function get data():ByteArray {
            return this._data;
        }
        public function set data(value:ByteArray):void {
            this._data = value;
        }

        //  Creates a copy of the SampleDataEvent object and sets each property's value to match that of the original.
        override public function clone():Event
        {
            return new SampleDataEvent(this.type, this.bubbles, this.cancelable, this.position, this.data);
        }

        //  Returns a string that contains all the properties of the SampleDataEvent object.
        override public function toString():String
        {
            return this.formatToString("SampleDataEvent","type","bubbles","cancelable","eventPhase","position","data");
        }
    }
}

