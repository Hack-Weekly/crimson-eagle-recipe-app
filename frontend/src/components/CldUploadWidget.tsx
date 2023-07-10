import { CldUploadWidget } from 'next-cloudinary';
 

<CldUploadWidget uploadPreset="<Upload Preset>">
  {({ open }) => {
    function handleOnClick(e: { preventDefault: () => void; }) {
      e.preventDefault();
      open();
    }
    return (
      <button className="button" onClick={handleOnClick}>
        Upload an Image
      </button>
    );
  }}
</CldUploadWidget>