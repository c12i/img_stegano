import {
  Button,
  Card,
  CardBody,
  Box,
  Center,
  Container,
  HStack,
  Heading,
  Input,
  Radio,
  RadioGroup,
  Tag,
  Text,
  Image,
  Link,
} from "@chakra-ui/react";
import { useCallback, useMemo, useRef, useState } from "react";
import { useDropzone } from "react-dropzone";
import { FaDownload } from "react-icons/fa";

import Nav from "./components/Nav";
import { getAsByteArray } from "./utils/files";

const App = () => {
  const { acceptedFiles, getRootProps, getInputProps } = useDropzone({
    accept: { "image/png": [".png"] },
    multiple: false,
  });
  const [encodedImage, setEncodedImage] = useState<Uint8Array>();
  const [decodedText, setDecodedText] = useState<string>("");
  const [action, setAction] = useState<string>("encode");
  const [value, setValue] = useState<string>("");
  const imageRef = useRef(null);
  const hiddenLinkRef = useRef<HTMLAnchorElement>(null);
  const downloadImage = useCallback(() => {}, []);
  const encodeText = useCallback(() => {
    import("../../stegano-wasm/pkg").then((mod) => {
      getAsByteArray(acceptedFiles[0]).then((buf) => {
        setEncodedImage(
          mod.encode_text(buf, acceptedFiles[0].type.split("/")[1], value),
        );
      });
    });
  }, [acceptedFiles, value]);
  const decodeText = useCallback(() => {
    import("../../stegano-wasm/pkg").then((mod) => {
      getAsByteArray(acceptedFiles[0]).then((buf) => {
        setDecodedText(mod.decode_text(buf));
      });
    });
  }, [setDecodedText, acceptedFiles]);
  const imageUrl = useMemo(() => {
    if (!encodedImage) return;
    const blob = new Blob([encodedImage.buffer], {
      type: acceptedFiles[0]?.type,
    });
    return URL.createObjectURL(blob);
  }, [acceptedFiles, encodedImage]);
  const files = acceptedFiles.map((file: any) => (
    <Tag key={file.path}>{file.path}</Tag>
  ));
  return (
    <Box>
      <Nav />
      <Container maxW="800px">
        <Heading py="2">wasm demo</Heading>
        <Card {...getRootProps()} cursor="pointer" py="10" px="10">
          <input {...getInputProps()} />
          <CardBody>
            <Text textAlign="center">
              Drag and drop or click to select file
            </Text>
          </CardBody>
        </Card>
        {!!acceptedFiles.length && <Center py="4">{files}</Center>}
        <RadioGroup
          py="4"
          onChange={(a) => {
            setAction(a);
            setDecodedText("");
            setEncodedImage(undefined);
          }}
          value={action}
        >
          <HStack>
            <Radio value="encode">Encode</Radio>
            <Radio value="decode">Decode</Radio>
          </HStack>
        </RadioGroup>
        {action === "encode" && (
          <Input
            onChange={(e) => setValue(e.target.value)}
            placeholder="Text to encode to image"
            value={value}
          />
        )}
        {action === "encode" ? (
          <Button
            disabled={!value && !acceptedFiles.length}
            my="4"
            onClick={encodeText}
          >
            Encode
          </Button>
        ) : (
          <Button onClick={decodeText} my="4">
            Decode
          </Button>
        )}
        {!!encodedImage?.length && (
          <HStack spacing="4">
            <Image
              boxSize="300px"
              src={imageUrl}
              alt="Dan Abramov"
              ref={imageRef}
            />
            <Link display="none" ref={hiddenLinkRef} />
            <Button
              as="a"
              download
              href={imageUrl}
              leftIcon={<FaDownload />}
              onClick={downloadImage}
              size="sm"
            >
              Download
            </Button>
          </HStack>
        )}
        {!!decodedText && <Input value={decodedText} readOnly />}
      </Container>
    </Box>
  );
};

export default App;
