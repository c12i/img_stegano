import {Box, Flex, Button, useColorModeValue, Stack, useColorMode, Heading} from '@chakra-ui/react'
import {FaMoon, FaSun} from 'react-icons/fa'

export default function Nav() {
  const { colorMode, toggleColorMode } = useColorMode()
  return (
		<Box bg={useColorModeValue('gray.100', 'gray.900')} px={4}>
			<Flex h={16} alignItems={'center'} justifyContent={'space-between'}>
				<Heading size="md">img_stegano</Heading>
				<Flex alignItems={'center'}>
					<Stack direction={'row'} spacing={7}>
						<Button onClick={toggleColorMode}>
							{colorMode === 'light' ? <FaMoon /> : <FaSun />}
						</Button>
					</Stack>
				</Flex>
			</Flex>
		</Box>
  )
}